use std::collections::HashMap;
use std::sync::Arc;

use bootstrappo::ops::drivers::HealthStatus;
use bootstrappo::ops::k8s::cache::ClusterCache;
use bootstrappo::ops::reconciler::plan::Plan;

pub trait PlanPort: Send + Sync {
    fn plan(&self) -> Option<Plan>;
    fn plan_error(&self) -> Option<String>;
}

pub trait HealthPort: Send + Sync {
    fn health(&self) -> HashMap<String, HealthStatus>;
    fn last_error(&self) -> Option<String>;
}

pub trait CachePort: Send + Sync {
    fn cache(&self) -> Option<ClusterCache>;
}

#[derive(Clone)]
pub struct PortSet {
    pub plan: Arc<dyn PlanPort>,
    pub health: Arc<dyn HealthPort>,
    pub cache: Arc<dyn CachePort>,
}

impl PortSet {
    pub fn empty() -> Self {
        Self {
            plan: Arc::new(NullPlanPort),
            health: Arc::new(NullHealthPort),
            cache: Arc::new(NullCachePort),
        }
    }
}

#[derive(Clone, Default)]
struct NullPlanPort;

impl PlanPort for NullPlanPort {
    fn plan(&self) -> Option<Plan> {
        None
    }

    fn plan_error(&self) -> Option<String> {
        None
    }
}

#[derive(Clone, Default)]
struct NullHealthPort;

impl HealthPort for NullHealthPort {
    fn health(&self) -> HashMap<String, HealthStatus> {
        HashMap::new()
    }

    fn last_error(&self) -> Option<String> {
        None
    }
}

#[derive(Clone, Default)]
struct NullCachePort;

impl CachePort for NullCachePort {
    fn cache(&self) -> Option<ClusterCache> {
        None
    }
}
