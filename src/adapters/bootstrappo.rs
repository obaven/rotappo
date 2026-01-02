use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;

use crate::ports::{CachePort, HealthPort, PlanPort, PortSet};

pub struct BootstrappoBackend {
    pub config: Arc<bootstrappo::config::Config>,
    pub config_path: PathBuf,
    pub plan_path: PathBuf,
    pub plan: Option<bootstrappo::ops::reconciler::plan::Plan>,
    pub plan_error: Option<String>,
    pub live_status: Option<crate::runtime::LiveStatus>,
    ports: PortSet,
}

impl BootstrappoBackend {
    pub fn from_env() -> Result<Self> {
        let config_path = std::env::var("BOOTSTRAPPO_CONFIG_PATH")
            .map(PathBuf::from)
            .ok();
        let plan_path = std::env::var("BOOTSTRAPPO_PLAN_PATH")
            .map(PathBuf::from)
            .ok();
        Self::from_paths(config_path, plan_path)
    }

    pub fn from_paths(
        config_path: Option<PathBuf>,
        plan_path: Option<PathBuf>,
    ) -> Result<Self> {
        let config_path = config_path.unwrap_or_else(|| {
            PathBuf::from("../bootstrappo/data/configs/bootstrap-config.yaml")
        });
        let config = bootstrappo::config::load_from_file(&config_path).with_context(|| {
            format!(
                "Failed to load Bootstrappo config at {}",
                config_path.display()
            )
        })?;

        let plan_path = plan_path.unwrap_or_else(|| {
            PathBuf::from("../bootstrappo/data/plans/bootstrap.v0-0-3.yaml")
        });
        let (plan, plan_error) = match bootstrappo::ops::reconciler::plan::Plan::load(&plan_path) {
            Ok(plan) => (Some(plan), None),
            Err(err) => (None, Some(err.to_string())),
        };
        let config = Arc::new(config);
        let live_status = Some(crate::runtime::LiveStatus::spawn(Arc::clone(&config)));
        let ports = BootstrappoPorts {
            plan: plan.clone(),
            plan_error: plan_error.clone(),
            live_status: live_status.clone(),
        }
        .into_portset();

        Ok(Self {
            config,
            config_path,
            plan_path,
            plan,
            plan_error,
            live_status,
            ports,
        })
    }

    pub fn runtime(&self) -> crate::runtime::Runtime {
        crate::runtime::Runtime::new_with_ports(
            crate::runtime::ActionRegistry::default(),
            self.ports.clone(),
        )
    }

    pub fn ports(&self) -> PortSet {
        self.ports.clone()
    }
}

#[derive(Clone)]
struct BootstrappoPorts {
    plan: Option<bootstrappo::ops::reconciler::plan::Plan>,
    plan_error: Option<String>,
    live_status: Option<crate::runtime::LiveStatus>,
}

impl BootstrappoPorts {
    fn into_portset(self) -> PortSet {
        let plan = Arc::new(self.clone());
        let health = Arc::new(self.clone());
        let cache = Arc::new(self);
        PortSet { plan, health, cache }
    }
}

impl PlanPort for BootstrappoPorts {
    fn plan(&self) -> Option<bootstrappo::ops::reconciler::plan::Plan> {
        self.plan.clone()
    }

    fn plan_error(&self) -> Option<String> {
        self.plan_error.clone()
    }
}

impl HealthPort for BootstrappoPorts {
    fn health(&self) -> std::collections::HashMap<String, bootstrappo::ops::drivers::HealthStatus> {
        self.live_status
            .as_ref()
            .map(|live| live.health())
            .unwrap_or_default()
    }

    fn last_error(&self) -> Option<String> {
        self.live_status.as_ref().and_then(|live| live.last_error())
    }
}

impl CachePort for BootstrappoPorts {
    fn cache(&self) -> Option<bootstrappo::ops::k8s::cache::ClusterCache> {
        self.live_status
            .as_ref()
            .and_then(|live| live.cache())
    }
}
