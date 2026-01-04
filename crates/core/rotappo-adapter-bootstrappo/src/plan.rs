use std::path::Path;
use std::sync::Arc;
use rotappo_adapter_bootstrappo::mapping;
use rotappo_domain::{Plan, PlanStepDef};
use rotappo_ports::PlanPort;
use super::health::LiveStatus;

#[derive(Clone)]
pub struct BootstrappoPlanPort {
    plan: Option<Plan>,
    raw_plan: Option<bootstrappo::ops::reconciler::plan::Plan>,
    plan_error: Option<String>,
    live_status: Option<LiveStatus>,
    config: Arc<bootstrappo::config::Config>,
}

impl BootstrappoPlanPort {
    pub fn load(
        plan_path: &Path,
        live_status: Option<LiveStatus>,
        config: Arc<bootstrappo::config::Config>,
    ) -> Self {
        let (raw_plan, plan_error) =
            match bootstrappo::ops::reconciler::plan::Plan::load(plan_path) {
                Ok(plan) => (Some(plan), None),
                Err(err) => (None, Some(err.to_string())),
            };
        let plan = raw_plan
            .as_ref()
            .map(|plan| map_plan(plan));
        Self {
            plan,
            raw_plan,
            plan_error,
            live_status,
            config,
        }
    }

    pub fn plan(&self) -> Option<Plan> {
        self.plan.clone()
    }

    pub fn plan_error(&self) -> Option<String> {
        self.plan_error.clone()
    }
}

impl PlanPort for BootstrappoPlanPort {
    fn plan(&self) -> Option<Plan> {
        self.plan.clone()
    }

    fn plan_error(&self) -> Option<String> {
        self.plan_error.clone()
    }

    fn step_readiness(&self) -> std::collections::HashMap<String, bool> {
        let mut readiness = std::collections::HashMap::new();
        let Some(raw_plan) = &self.raw_plan else {
            return readiness;
        };
        let cache = self.live_status.as_ref().and_then(|live| live.cache());
        for step in &raw_plan.steps {
            let ready = if let Some(cache) = &cache {
                mapping::gates_ready(cache, step, Some(self.config.as_ref()))
            } else {
                step.gates.is_empty()
            };
            readiness.insert(step.id.clone(), ready);
        }
        readiness
    }
}

fn map_plan(plan: &bootstrappo::ops::reconciler::plan::Plan) -> Plan {
    let spec_map = mapping::driver_specs();
    let steps = plan
        .steps
        .iter()
        .map(|step| {
            let (domain, namespace) = spec_map
                .get(step.id.as_str())
                .cloned()
                .unwrap_or_else(|| ("unknown".to_string(), None));
            let pod = mapping::derive_pod_value(step, namespace.as_deref());
            PlanStepDef {
                id: step.id.clone(),
                kind: step.kind.clone(),
                depends_on: step.depends_on.clone(),
                provides: step.provides.clone(),
                domain,
                pod,
                has_gates: !step.gates.is_empty(),
            }
        })
        .collect();
    Plan { steps }
}
