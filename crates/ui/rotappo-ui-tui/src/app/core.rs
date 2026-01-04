//! Core application state for the TUI.

use ratatui::widgets::ListState;
use std::path::PathBuf;
use std::time::Instant;

use rotappo_application::Runtime;
use rotappo_domain::{ActionId, ActionSafety};
use crate::layout::LayoutPolicy;
use crate::state::UiState;
use rotappo_ports::PortSet;

/// External context required to run the TUI.
#[derive(Clone)]
pub struct AppContext {
    pub host_domain: String,
    pub config_path: PathBuf,
    pub plan_path: PathBuf,
    pub plan_error: Option<String>,
    pub live_status_error: Option<String>,
    pub ports: PortSet,
}

impl AppContext {
    /// Build a minimal context without plan/live errors.
    pub fn new(
        host_domain: impl Into<String>,
        config_path: impl Into<PathBuf>,
        plan_path: impl Into<PathBuf>,
        ports: PortSet,
    ) -> Self {
        Self {
            host_domain: host_domain.into(),
            config_path: config_path.into(),
            plan_path: plan_path.into(),
            plan_error: None,
            live_status_error: None,
            ports,
        }
    }
}

/// Main TUI application state.
///
/// # Examples
/// ```rust,no_run
/// use rotappo_application::Runtime;
/// use rotappo_domain::ActionRegistry;
/// use rotappo_ui_tui::app::App;
/// use rotappo_ui_tui::app::AppContext;
/// use rotappo_ports::PortSet;
///
/// let runtime = Runtime::new_with_ports(ActionRegistry::default(), PortSet::empty());
/// let context = AppContext::new(\"localhost\", \"config.yml\", \"plan.yml\", PortSet::empty());
/// let app = App::new(runtime, context);
/// assert!(!app.should_quit);
/// ```
pub struct App {
    pub runtime: Runtime,
    pub context: AppContext,
    pub action_state: ListState,
    pub confirm: Option<ConfirmPrompt>,
    pub last_refresh: Instant,
    pub should_quit: bool,
    pub ui: UiState,
    pub layout_policy: LayoutPolicy,
}

/// Confirmation prompt details for high-risk actions.
#[derive(Debug, Clone)]
pub struct ConfirmPrompt {
    pub action_id: ActionId,
    pub label: String,
    pub safety: ActionSafety,
}
