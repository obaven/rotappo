//! Panel identifiers for UI-core contracts.

/// Panels displayed in the UI shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiPanelId {
    ActionProgress,
    Snapshot,
    Capabilities,
    ActionSteps,
    Actions,
    Settings,
    LogControls,
    Problems,
    Logs,
    Help,
}
