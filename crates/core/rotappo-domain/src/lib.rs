//! Domain models and invariants.

pub mod actions;
pub mod events;
pub mod health;
pub mod plan;
pub mod snapshot;

pub use actions::{Action, ActionId, ActionRegistry, ActionSafety};
pub use events::{Event, EventBus, EventLevel};
pub use health::{ComponentHealthStatus, HealthSnapshot};
pub use plan::{Plan, PlanStepDef};
pub use snapshot::{
    now_millis, ActionStatus, Capability, CapabilityStatus, HealthStatus, PlanStep,
    PlanStepStatus, PlanSummary, Snapshot,
};
