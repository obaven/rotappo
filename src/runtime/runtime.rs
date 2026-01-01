use anyhow::{anyhow, Result};

use crate::runtime::actions::{ActionId, ActionRegistry, ActionSafety};
use crate::runtime::events::{Event, EventBus, EventLevel};
use crate::ports::PortSet;
use crate::runtime::snapshot::{ActionStatus, PlanStep, PlanStepStatus, Snapshot};

pub struct Runtime {
    registry: ActionRegistry,
    snapshot: Snapshot,
    events: EventBus,
    refresh_count: u64,
    plan: Option<bootstrappo::ops::reconciler::plan::Plan>,
    ports: PortSet,
}

impl Runtime {
    pub fn new(registry: ActionRegistry) -> Self {
        Self::new_with_ports(registry, PortSet::empty())
    }

    pub fn new_with_ports(registry: ActionRegistry, ports: PortSet) -> Self {
        let mut events = EventBus::default();
        events.push(Event::new(EventLevel::Info, "Runtime initialized"));
        let plan = ports.plan.plan();
        let snapshot = match plan.as_ref() {
            Some(plan) => Self::snapshot_from_plan(plan),
            None => Snapshot::new_default(),
        };

        let mut runtime = Self {
            registry,
            snapshot,
            events,
            refresh_count: 0,
            plan,
            ports,
        };
        runtime.snapshot.update_plan_summary_from_steps();
        runtime
    }

    pub fn registry(&self) -> &ActionRegistry {
        &self.registry
    }

    pub fn snapshot(&self) -> &Snapshot {
        &self.snapshot
    }

    pub fn events(&self) -> &EventBus {
        &self.events
    }

    pub fn events_mut(&mut self) -> &mut EventBus {
        &mut self.events
    }

    pub fn refresh_snapshot(&mut self) {
        self.refresh_count = self.refresh_count.saturating_add(1);
        if !self.snapshot.plan_steps.is_empty() {
            self.update_plan_statuses();
            self.sync_capabilities_from_steps();
        }
        self.snapshot.touch();
    }

    pub fn trigger_action(&mut self, action_id: ActionId) -> Result<()> {
        let action = self
            .registry
            .get(action_id)
            .ok_or_else(|| anyhow!("Unknown action: {action_id}"))?;

        if action.safety == ActionSafety::Destructive {
            self.events.push(Event::new(
                EventLevel::Warn,
                format!("Destructive action queued: {}", action.label),
            ));
        }

        self.snapshot.mark_action(action_id, ActionStatus::Running);
        self.events.push(Event::new(
            EventLevel::Info,
            format!("Started action: {}", action.label),
        ));

        self.snapshot.mark_action(action_id, ActionStatus::Succeeded);
        self.events.push(Event::new(
            EventLevel::Info,
            format!("Completed action: {}", action.label),
        ));

        self.snapshot.touch();
        Ok(())
    }

    fn snapshot_from_plan(plan: &bootstrappo::ops::reconciler::plan::Plan) -> Snapshot {
        let mut snapshot = Snapshot::new_default();
        let spec_map = Self::driver_specs();
        snapshot.plan_steps = plan
            .steps
            .iter()
            .map(|step| {
                let (domain, namespace) = spec_map
                    .get(step.id.as_str())
                    .cloned()
                    .unwrap_or_else(|| ("unknown".to_string(), None));
                let pod = Self::derive_pod_value(step, namespace.as_deref());
                PlanStep {
                    id: step.id.clone(),
                    kind: step.kind.clone(),
                    depends_on: step.depends_on.clone(),
                    provides: step.provides.clone(),
                    status: PlanStepStatus::Pending,
                    domain,
                    pod,
                }
            })
            .collect();

        snapshot.capabilities = plan
            .steps
            .iter()
            .flat_map(|step| step.provides.iter().cloned())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .map(|name| super::snapshot::Capability {
                name,
                status: super::snapshot::CapabilityStatus::Degraded,
            })
            .collect();

        snapshot.update_plan_summary_from_steps();
        snapshot
    }

    fn update_plan_statuses(&mut self) {
        let plan = match &self.plan {
            Some(plan) => plan,
            None => {
                self.snapshot.update_plan_summary_from_steps();
                return;
            }
        };

        let health_map = self.ports.health.health();
        let cache = self.ports.cache.cache();

        let step_map: std::collections::HashMap<_, _> =
            plan.steps.iter().map(|step| (step.id.as_str(), step)).collect();

        let statuses: Vec<PlanStepStatus> = self
            .snapshot
            .plan_steps
            .iter()
            .map(|step| {
                let blocked = step.depends_on.iter().any(|dep| {
                    self.snapshot
                        .plan_steps
                        .iter()
                        .any(|other| other.id == *dep && other.status != PlanStepStatus::Succeeded)
                });

                let mut status = if blocked {
                    PlanStepStatus::Blocked
                } else {
                    PlanStepStatus::Pending
                };

                if let Some(def) = step_map.get(step.id.as_str()) {
                    if let Some(cache) = &cache {
                        if Self::gates_ready(cache, def) {
                            status = PlanStepStatus::Succeeded;
                        }
                    } else if def.gates.is_empty() && !blocked {
                        status = PlanStepStatus::Succeeded;
                    }
                }

                if let Some(health) = health_map.get(&step.id) {
                    status = match health {
                        bootstrappo::ops::drivers::HealthStatus::Healthy => status,
                        bootstrappo::ops::drivers::HealthStatus::Degraded(_) => {
                            PlanStepStatus::Running
                        }
                        bootstrappo::ops::drivers::HealthStatus::Unhealthy(_) => {
                            PlanStepStatus::Failed
                        }
                    };
                }

                status
            })
            .collect();

        for (step, status) in self.snapshot.plan_steps.iter_mut().zip(statuses) {
            step.status = status;
        }
        self.snapshot.update_plan_summary_from_steps();
    }

    fn sync_capabilities_from_steps(&mut self) {
        let completed: std::collections::BTreeSet<String> = self
            .snapshot
            .plan_steps
            .iter()
            .filter(|step| step.status == PlanStepStatus::Succeeded)
            .flat_map(|step| step.provides.iter().cloned())
            .collect();

        for capability in &mut self.snapshot.capabilities {
            if completed.contains(&capability.name) {
                capability.status = super::snapshot::CapabilityStatus::Ready;
            } else if capability.status == super::snapshot::CapabilityStatus::Ready {
                capability.status = super::snapshot::CapabilityStatus::Degraded;
            }
        }
    }

    fn driver_specs() -> std::collections::HashMap<String, (String, Option<String>)> {
        bootstrappo::components::registry::get_all_specs()
            .into_iter()
            .map(|spec| {
                (
                    spec.name.to_string(),
                    (
                        spec.domain.to_string(),
                        spec.namespace.map(|namespace| namespace.to_string()),
                    ),
                )
            })
            .collect()
    }

    fn derive_pod_value(
        step: &bootstrappo::ops::reconciler::plan::Step,
        namespace: Option<&str>,
    ) -> Option<String> {
        let gate_label = step.gates.iter().find_map(|gate| match gate {
            bootstrappo::ops::reconciler::plan::Gate::DaemonsetReady { namespace, name } => {
                Some(format!("{}/{}", namespace, name))
            }
            bootstrappo::ops::reconciler::plan::Gate::DeploymentReady { namespace, name } => {
                Some(format!("{}/{}", namespace, name))
            }
            bootstrappo::ops::reconciler::plan::Gate::StatefulsetReady { namespace, name } => {
                Some(format!("{}/{}", namespace, name))
            }
            _ => None,
        });

        gate_label.or_else(|| namespace.map(|ns| format!("{}/{}", ns, step.id)))
    }

    fn gates_ready(
        cache: &bootstrappo::ops::k8s::cache::ClusterCache,
        step: &bootstrappo::ops::reconciler::plan::Step,
    ) -> bool {
        if step.gates.is_empty() {
            return true;
        }
        for gate in &step.gates {
            match gate {
                bootstrappo::ops::reconciler::plan::Gate::DaemonsetReady { namespace, name } => {
                    if !cache.is_daemonset_ready(namespace, name) {
                        return false;
                    }
                }
                bootstrappo::ops::reconciler::plan::Gate::DeploymentReady { namespace, name } => {
                    if !cache.is_deployment_ready(namespace, name) {
                        return false;
                    }
                }
                bootstrappo::ops::reconciler::plan::Gate::StatefulsetReady { namespace, name } => {
                    if !cache.is_statefulset_ready(namespace, name) {
                        return false;
                    }
                }
                bootstrappo::ops::reconciler::plan::Gate::CrdEstablished { name } => {
                    if !cache.is_crd_established(name) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(ActionRegistry::default())
    }
}
