use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionId {
    Validate,
    Reconcile,
    Rotate,
    Nuke,
    Debug,
}

impl ActionId {
    pub fn as_str(self) -> &'static str {
        match self {
            ActionId::Validate => "validate",
            ActionId::Reconcile => "reconcile",
            ActionId::Rotate => "rotate",
            ActionId::Nuke => "nuke",
            ActionId::Debug => "debug",
        }
    }
}

impl fmt::Display for ActionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionSafety {
    Safe,
    Guarded,
    Destructive,
}

impl ActionSafety {
    pub fn as_str(self) -> &'static str {
        match self {
            ActionSafety::Safe => "safe",
            ActionSafety::Guarded => "guarded",
            ActionSafety::Destructive => "destructive",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: ActionId,
    pub label: &'static str,
    pub description: &'static str,
    pub requires_confirmation: bool,
    pub safety: ActionSafety,
}

impl Action {
    pub const fn new(
        id: ActionId,
        label: &'static str,
        description: &'static str,
        requires_confirmation: bool,
        safety: ActionSafety,
    ) -> Self {
        Self {
            id,
            label,
            description,
            requires_confirmation,
            safety,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActionRegistry {
    actions: Vec<Action>,
}

impl ActionRegistry {
    pub fn bootstrappo_default() -> Self {
        Self {
            actions: vec![
                Action::new(
                    ActionId::Validate,
                    "Plan Validate",
                    "Validate desired state without mutating infrastructure.",
                    false,
                    ActionSafety::Safe,
                ),
                Action::new(
                    ActionId::Reconcile,
                    "Reconcile",
                    "Apply desired state to the cluster with full drift repair.",
                    true,
                    ActionSafety::Guarded,
                ),
                Action::new(
                    ActionId::Rotate,
                    "Rotate Secrets",
                    "Rotate managed credentials and reconcile dependent services.",
                    true,
                    ActionSafety::Guarded,
                ),
                Action::new(
                    ActionId::Debug,
                    "Debug Snapshot",
                    "Collect diagnostics for plan drift and action failures.",
                    false,
                    ActionSafety::Safe,
                ),
                Action::new(
                    ActionId::Nuke,
                    "Nuke",
                    "Destroy all managed resources in the target workspace.",
                    true,
                    ActionSafety::Destructive,
                ),
            ],
        }
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    pub fn get(&self, id: ActionId) -> Option<&Action> {
        self.actions.iter().find(|action| action.id == id)
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::bootstrappo_default()
    }
}
