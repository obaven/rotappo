use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::actions::ActionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionStepStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Blocked,
}

impl ActionStepStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ActionStepStatus::Pending => "pending",
            ActionStepStatus::Running => "running",
            ActionStepStatus::Succeeded => "completed",
            ActionStepStatus::Failed => "failed",
            ActionStepStatus::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionStep {
    pub id: String,
    pub kind: String,
    pub depends_on: Vec<String>,
    pub provides: Vec<String>,
    pub status: ActionStepStatus,
    pub domain: String,
    pub pod: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSummary {
    pub total: u32,
    pub completed: u32,
    pub in_progress: u32,
    pub blocked: u32,
    pub pending: u32,
}

impl ActionSummary {
    pub fn percent_complete(&self) -> u16 {
        if self.total == 0 {
            return 0;
        }
        let ratio = self.completed.saturating_mul(100) / self.total;
        ratio.min(100) as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unavailable,
}

impl HealthStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Unavailable => "unavailable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityStatus {
    Ready,
    Degraded,
    Offline,
}

impl CapabilityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CapabilityStatus::Ready => "ready",
            CapabilityStatus::Degraded => "degraded",
            CapabilityStatus::Offline => "offline",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub status: CapabilityStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

impl ActionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ActionStatus::Pending => "pending",
            ActionStatus::Running => "running",
            ActionStatus::Succeeded => "succeeded",
            ActionStatus::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub action: ActionSummary,
    pub action_steps: Vec<ActionStep>,
    pub capabilities: Vec<Capability>,
    pub health: HealthStatus,
    pub last_updated_ms: u64,
    pub last_action: Option<ActionId>,
    pub last_action_status: Option<ActionStatus>,
}

impl Snapshot {
    pub fn new_default() -> Self {
        Self {
            action: ActionSummary {
                total: 12,
                completed: 3,
                in_progress: 2,
                blocked: 1,
                pending: 6,
            },
            action_steps: vec![],
            capabilities: vec![
                Capability {
                    name: "Action Snapshot".to_string(),
                    status: CapabilityStatus::Ready,
                },
                Capability {
                    name: "Action Router".to_string(),
                    status: CapabilityStatus::Degraded,
                },
                Capability {
                    name: "Event Stream".to_string(),
                    status: CapabilityStatus::Ready,
                },
                Capability {
                    name: "Desktop Bridge".to_string(),
                    status: CapabilityStatus::Offline,
                },
            ],
            health: HealthStatus::Degraded,
            last_updated_ms: now_millis(),
            last_action: None,
            last_action_status: None,
        }
    }

    pub fn touch(&mut self) {
        self.last_updated_ms = now_millis();
    }

    pub fn mark_action(&mut self, action: ActionId, status: ActionStatus) {
        self.last_action = Some(action);
        self.last_action_status = Some(status);
        self.touch();
    }

    pub fn update_action_summary_from_steps(&mut self) {
        if self.action_steps.is_empty() {
            return;
        }
        let total = self.action_steps.len() as u32;
        let mut completed = 0;
        let mut in_progress = 0;
        let mut blocked = 0;
        let mut pending = 0;
        for step in &self.action_steps {
            match step.status {
                ActionStepStatus::Succeeded => completed += 1,
                ActionStepStatus::Running => in_progress += 1,
                ActionStepStatus::Blocked => blocked += 1,
                ActionStepStatus::Pending => pending += 1,
                _ => {}
            }
        }
        self.action.total = total;
        self.action.completed = completed;
        self.action.in_progress = in_progress;
        self.action.blocked = blocked;
        self.action.pending = pending;
    }
}

pub fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
