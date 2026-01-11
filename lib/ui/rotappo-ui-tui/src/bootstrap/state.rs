use bootstrappo::application::events::InteractiveCommand;
use bootstrappo::application::flows::reconcile::visualize::LayerType;
use std::collections::{HashSet, VecDeque};

use rotappo_domain::Event;

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum FocusTarget {
    #[default]
    Tree,
    Status,
}

impl FocusTarget {
    pub fn toggle(self) -> Self {
        match self {
            FocusTarget::Tree => FocusTarget::Status,
            FocusTarget::Status => FocusTarget::Tree,
        }
    }
}

#[derive(Default)]
pub struct BootstrapUiState {
    pub show_summary: bool,
    pub completed_seen: bool,
    pub show_logs: bool,
    pub focus: FocusTarget,
    pub tree_selected: usize,
    pub tree_scroll: usize,
    pub collapsed_layers: HashSet<LayerType>,
    pub status_selected: usize,
    pub status_scroll: usize,
    pub expanded_components: HashSet<String>,
    pub menu_state: MenuState,
    pub paused: bool,
    pub log_events: VecDeque<Event>,
    pub log_scroll: usize,
    pub log_view_height: usize,
}

#[derive(Default)]
pub struct MenuState {
    pub active: bool,
    pub selected: usize,
    pub confirming: bool,
    pub confirmation_message: String,
    pub pending_command: Option<InteractiveCommand>,
    pub timeout_input: Option<String>,
}

impl MenuState {
    pub fn open(&mut self) {
        self.active = true;
        self.selected = 0;
        self.confirming = false;
        self.timeout_input = None;
    }

    pub fn confirm(&mut self, message: String, cmd: InteractiveCommand) {
        self.confirming = true;
        self.confirmation_message = message;
        self.pending_command = Some(cmd);
    }

    pub fn clear(&mut self) {
        self.active = false;
        self.selected = 0;
        self.confirming = false;
        self.confirmation_message.clear();
        self.pending_command = None;
        self.timeout_input = None;
    }
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Skip,
    Retry,
    AdjustTimeout,
    ViewLogs,
    ToggleExpand,
    Pause,
    Resume,
    Cancel,
}

impl MenuAction {
    pub fn label(self) -> &'static str {
        match self {
            MenuAction::Skip => "Skip Component",
            MenuAction::Retry => "Retry Component",
            MenuAction::AdjustTimeout => "Adjust Timeout",
            MenuAction::ViewLogs => "View Logs",
            MenuAction::ToggleExpand => "Expand Details",
            MenuAction::Pause => "Pause Bootstrap",
            MenuAction::Resume => "Resume Bootstrap",
            MenuAction::Cancel => "Cancel Bootstrap",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            MenuAction::Skip => "Mark as deferred and continue",
            MenuAction::Retry => "Retry selected component",
            MenuAction::AdjustTimeout => "Change readiness timeout",
            MenuAction::ViewLogs => "Show recent bootstrap logs",
            MenuAction::ToggleExpand => "Show readiness details",
            MenuAction::Pause => "Pause component execution",
            MenuAction::Resume => "Resume component execution",
            MenuAction::Cancel => "Stop bootstrap",
        }
    }
}
