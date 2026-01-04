mod health;
pub mod mapping;
mod plan;

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;

use rotappo_ports::{LogPort, PortSet};
use rotappo_domain::Event;

pub use health::LiveStatus;

pub struct BootstrappoBackend {
    pub config: Arc<bootstrappo::config::Config>,
    pub config_path: PathBuf,
    pub plan_path: PathBuf,
    pub plan: Option<rotappo_domain::Plan>,
    pub plan_error: Option<String>,
    pub live_status: Option<LiveStatus>,
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
        let config = Arc::new(config);
        let live_status = Some(LiveStatus::spawn(Arc::clone(&config)));
        let plan_port = plan::BootstrappoPlanPort::load(
            &plan_path,
            live_status.clone(),
            Arc::clone(&config),
        );
        let plan = plan_port.plan();
        let plan_error = plan_port.plan_error();
        let health_port = health::BootstrappoHealthPort::new(live_status.clone());
        let ports = PortSet {
            plan: Arc::new(plan_port),
            health: Arc::new(health_port),
            logs: Arc::new(BootstrappoLogPort),
        };

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

    pub fn ports(&self) -> PortSet {
        self.ports.clone()
    }
}

#[derive(Clone, Copy)]
struct BootstrappoLogPort;

impl LogPort for BootstrappoLogPort {
    fn drain_events(&self) -> Vec<Event> {
        Vec::new()
    }
}
