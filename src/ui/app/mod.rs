use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{layout::Rect, widgets::ListState};
use std::time::{Duration, Instant};

use crate::adapters::bootstrappo::BootstrappoBackend;
use crate::runtime::{ActionId, ActionSafety, Event, EventLevel, Runtime};
use crate::ui::state::UiState;

mod actions;
mod collapse;
mod hover;
mod input;
mod scroll;
mod tooltips;

const COLLAPSED_HEIGHT: u16 = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PanelId {
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

pub struct App {
    pub backend: BootstrappoBackend,
    pub runtime: Runtime,
    pub action_state: ListState,
    pub confirm: Option<ConfirmPrompt>,
    pub last_refresh: Instant,
    pub should_quit: bool,
    pub ui: UiState,
}

#[derive(Debug, Clone)]
pub struct ConfirmPrompt {
    pub action_id: ActionId,
    pub label: String,
    pub safety: ActionSafety,
}

impl App {
    pub fn new(backend: BootstrappoBackend) -> Self {
        let mut runtime = backend.runtime();
        runtime.events_mut().push(Event::new(
            EventLevel::Info,
            format!(
                "Connected to Bootstrappo ({})",
                backend.config.network.host_domain
            ),
        ));
        runtime.events_mut().push(Event::new(
            EventLevel::Info,
            format!("Plan path: {}", backend.plan_path.display()),
        ));
        if let Some(error) = &backend.plan_error {
            runtime.events_mut().push(Event::new(
                EventLevel::Warn,
                format!("Plan load failed: {}", error),
            ));
        }
        if let Some(live) = &backend.live_status {
            if let Some(error) = live.last_error() {
                runtime.events_mut().push(Event::new(
                    EventLevel::Warn,
                    format!("Live status unavailable: {}", error),
                ));
            }
        }

        let mut action_state = ListState::default();
        if !runtime.registry().actions().is_empty() {
            action_state.select(Some(0));
        }

        Self {
            backend,
            runtime,
            action_state,
            confirm: None,
            last_refresh: Instant::now(),
            should_quit: false,
            ui: UiState::new(),
        }
    }

    pub fn on_tick(&mut self) {
        if self.ui.auto_refresh && self.last_refresh.elapsed() >= Duration::from_secs(1) {
            self.runtime.refresh_snapshot();
            self.last_refresh = Instant::now();
        }

        let hold_trigger = if let Some(hold) = &mut self.ui.hold_state {
            if !hold.triggered && hold.started_at.elapsed() >= Duration::from_secs(3) {
                hold.triggered = true;
                Some(hold.key)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(key) = hold_trigger {
            match key {
                'p' => self.pin_tooltip(),
                'u' => self.unpin_tooltip(),
                _ => {}
            }
        }

        if !self.ui.log_paused && self.ui.last_log_emit.elapsed() >= self.ui.log_interval {
            self.runtime.events_mut().push(Event::new(
                EventLevel::Info,
                "Event stream heartbeat",
            ));
            self.ui.last_log_emit = Instant::now();
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if self.confirm.is_some() {
            return self.handle_confirm_key(key);
        }
        if self.handle_hold_key(&key) {
            return Ok(());
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('r') => self.runtime.refresh_snapshot(),
            KeyCode::Char('f') => self.ui.log_filter = self.ui.log_filter.next(),
            KeyCode::Char('s') => self.toggle_settings_panel(),
            KeyCode::Char('w') => self.ui.auto_refresh = !self.ui.auto_refresh,
            KeyCode::Char('a') => self.handle_settings_shortcut(true),
            KeyCode::Char('c') => self.handle_settings_shortcut(false),
            KeyCode::Up | KeyCode::Char('k') => self.select_previous_action(),
            KeyCode::Down | KeyCode::Char('j') => self.select_next_action(),
            KeyCode::Enter => self.trigger_selected_action()?,
            _ => {}
        }
        Ok(())
    }

    fn handle_settings_shortcut(&mut self, apply: bool) {
        if self.ui.collapsed_settings {
            return;
        }
        self.ui.settings_selected = if apply { 0 } else { 1 };
        let message = if apply {
            "Settings: apply (shortcut)"
        } else {
            "Settings: cancel (shortcut)"
        };
        self.runtime
            .events_mut()
            .push(Event::new(EventLevel::Info, message));
    }

    pub fn close_log_menu(&mut self) {
        self.ui.log_menu_pinned = false;
        self.ui.log_menu_mode = None;
        self.ui.log_menu_len = 0;
        self.ui.log_menu_area = Rect::default();
        self.ui.log_menu_hover_index = None;
    }
}
