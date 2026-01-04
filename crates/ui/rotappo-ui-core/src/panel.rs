//! Panel identifiers for UI-core contracts.

/// Panels displayed in the UI shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiPanelId {
    PlanProgress,
    Snapshot,
    Capabilities,
    PlanSteps,
    Actions,
    Settings,
    LogControls,
    Problems,
    Logs,
    Help,
}
