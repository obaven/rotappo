//! Domain models and invariants.

pub mod actions;
pub mod assembly;
pub mod events;
pub mod health;
pub mod snapshot;

pub use actions::{ActionDefinition, ActionId, ActionRegistry, ActionSafety};
pub use assembly::{Assembly, AssemblyStepDef};
pub use events::{Event, EventBus, EventLevel};
pub use health::{ComponentHealthStatus, HealthSnapshot};
pub use snapshot::{
    ActionStatus, AssemblyStep, AssemblyStepStatus, AssemblySummary, Capability, CapabilityStatus,
    HealthStatus, Snapshot, now_millis,
};
