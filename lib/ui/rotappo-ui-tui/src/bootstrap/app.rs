use crate::bootstrap::panels::dependency_tree::{TreeLine, build_tree_lines};
use crate::bootstrap::panels::{dependency_tree, header, logs, menu, status, summary};
use crate::bootstrap::state::{BootstrapUiState, FocusTarget, MenuAction};
use crate::bootstrap::utils::find_dependents;
use anyhow::Result;
use bootstrappo::application::events::InteractiveCommand;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Frame;
use rotappo_ports::{ComponentStatus, PortSet};
use std::time::Duration;

const LOG_MAX_EVENTS: usize = 500;

pub struct BootstrapApp {
    pub ports: PortSet,
    pub ui: BootstrapUiState,
    pub should_quit: bool,
}

impl BootstrapApp {
    pub fn new(ports: PortSet) -> Self {
        Self {
            ports,
            ui: BootstrapUiState::default(),
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self) {
        self.refresh_logs();
        let status = self.ports.bootstrap.bootstrap_status();
        if status.total_duration.is_some() && !self.ui.completed_seen {
            self.ui.show_summary = true;
            self.ui.completed_seen = true;
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if self.ui.show_logs {
            return self.handle_logs_input(key);
        }
        if self.ui.menu_state.active {
            return self.handle_menu_input(key);
        }

        if self.ui.show_summary {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('b') => self.ui.show_summary = false,
                _ => {}
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('m') => self.ui.menu_state.open(),
            KeyCode::Char('e') => self.toggle_expand_selected(),
            KeyCode::Char('c') => self.toggle_layer_collapse(),
            KeyCode::Tab => self.ui.focus = self.ui.focus.toggle(),
            KeyCode::Up => self.move_selection(-1),
            KeyCode::Down => self.move_selection(1),
            KeyCode::PageUp => self.scroll(-5),
            KeyCode::PageDown => self.scroll(5),
            _ => {}
        }
        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        if self.ui.show_summary {
            summary::render(frame, frame.area(), &self.ports);
        } else {
            let size = frame.area();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5),
                    Constraint::Percentage(55),
                    Constraint::Percentage(45),
                ])
                .split(size);

            header::render(frame, layout[0], &self.ports);
            dependency_tree::render(frame, layout[1], &self.ports, &self.ui);
            status::render(frame, layout[2], &self.ports, &mut self.ui);

            if self.ui.menu_state.active {
                let actions = self.menu_actions();
                menu::render(frame, size, &self.ports, &self.ui, &actions);
            }
        }

        if self.ui.show_logs {
            logs::render(frame, frame.area(), &mut self.ui);
        }
    }

    fn handle_menu_input(&mut self, key: KeyEvent) -> Result<()> {
        if self.ui.menu_state.confirming {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if let Some(cmd) = self.ui.menu_state.pending_command.take() {
                        self.ports.bootstrap.send_command(cmd)?;
                    }
                    self.ui.menu_state.clear();
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.ui.menu_state.clear();
                }
                _ => {}
            }
            return Ok(());
        }

        if self.ui.menu_state.timeout_input.is_some() {
            return self.handle_timeout_input(key);
        }

        match key.code {
            KeyCode::Up => {
                self.ui.menu_state.selected = self.ui.menu_state.selected.saturating_sub(1)
            }
            KeyCode::Down => {
                self.ui.menu_state.selected = self.ui.menu_state.selected.saturating_add(1)
            }
            KeyCode::Enter => self.execute_menu_action()?,
            KeyCode::Esc => self.ui.menu_state.clear(),
            _ => {}
        }
        Ok(())
    }

    fn handle_timeout_input(&mut self, key: KeyEvent) -> Result<()> {
        let Some(input) = self.ui.menu_state.timeout_input.as_mut() else {
            return Ok(());
        };

        match key.code {
            KeyCode::Enter => {
                let value = input.parse::<u64>().unwrap_or(0);
                if let Some(component) = self.selected_component_id() {
                    let cmd = InteractiveCommand::AdjustTimeout {
                        id: component,
                        new_timeout: Duration::from_secs(value),
                    };
                    self.ports.bootstrap.send_command(cmd)?;
                }
                self.ui.menu_state.clear();
            }
            KeyCode::Char(ch) if ch.is_ascii_digit() => input.push(ch),
            KeyCode::Backspace => {
                input.pop();
            }
            KeyCode::Esc => self.ui.menu_state.clear(),
            _ => {}
        }
        Ok(())
    }

    fn execute_menu_action(&mut self) -> Result<()> {
        let actions = self.menu_actions();
        let Some(action) = actions.get(self.ui.menu_state.selected).cloned() else {
            return Ok(());
        };

        match action {
            MenuAction::Skip => {
                let Some(component) = self.selected_component_id() else {
                    return Ok(());
                };
                let dependents =
                    find_dependents(self.ports.bootstrap.dependency_graph(), &component);
                self.ui.menu_state.confirm(
                    format!(
                        "Skip component: {}?\nThis will also defer: {}",
                        component,
                        if dependents.is_empty() {
                            "none".to_string()
                        } else {
                            dependents.join(", ")
                        }
                    ),
                    InteractiveCommand::SkipComponent { id: component },
                );
            }
            MenuAction::Retry => {
                if let Some(component) = self.selected_component_id() {
                    self.ports
                        .bootstrap
                        .send_command(InteractiveCommand::RetryComponent { id: component })?;
                }
                self.ui.menu_state.clear();
            }
            MenuAction::AdjustTimeout => {
                self.ui.menu_state.timeout_input = Some(String::new());
            }
            MenuAction::ViewLogs => {
                self.ui.show_logs = true;
                self.ui.log_scroll = 0;
                self.ui.menu_state.clear();
            }
            MenuAction::Pause => {
                self.ports
                    .bootstrap
                    .send_command(InteractiveCommand::PauseBootstrap)?;
                self.ui.paused = true;
                self.ui.menu_state.clear();
            }
            MenuAction::Resume => {
                self.ports
                    .bootstrap
                    .send_command(InteractiveCommand::ResumeBootstrap)?;
                self.ui.paused = false;
                self.ui.menu_state.clear();
            }
            MenuAction::Cancel => {
                self.ui.menu_state.confirm(
                    "Cancel bootstrap? This will stop execution.".to_string(),
                    InteractiveCommand::CancelBootstrap,
                );
            }
            MenuAction::ToggleExpand => {
                self.toggle_expand_selected();
                self.ui.menu_state.clear();
            }
        }

        Ok(())
    }

    fn selected_component_id(&self) -> Option<String> {
        let steps = self.ports.bootstrap.dependency_graph().steps.as_slice();
        steps
            .get(self.ui.status_selected)
            .map(|step| step.id.clone())
    }

    fn menu_actions(&self) -> Vec<MenuAction> {
        let Some(component) = self.selected_component_id() else {
            return vec![MenuAction::Cancel];
        };
        let state = self.ports.bootstrap.component_states();
        let status = state.get(&component).map(|s| s.status);

        let mut actions = Vec::new();
        match status {
            Some(ComponentStatus::Running) => {
                actions.push(MenuAction::Skip);
                actions.push(MenuAction::AdjustTimeout);
                actions.push(MenuAction::ViewLogs);
                actions.push(MenuAction::ToggleExpand);
            }
            Some(ComponentStatus::Failed) => {
                actions.push(MenuAction::Retry);
                actions.push(MenuAction::ViewLogs);
                actions.push(MenuAction::ToggleExpand);
            }
            Some(ComponentStatus::Deferred) => actions.push(MenuAction::Retry),
            Some(ComponentStatus::Complete) => {
                actions.push(MenuAction::ViewLogs);
                actions.push(MenuAction::ToggleExpand);
            }
            _ => {}
        }

        if self.ui.paused {
            actions.push(MenuAction::Resume);
        } else {
            actions.push(MenuAction::Pause);
        }
        actions.push(MenuAction::Cancel);

        actions
    }

    fn move_selection(&mut self, delta: i32) {
        match self.ui.focus {
            FocusTarget::Tree => {
                let total_lines = build_tree_lines(
                    self.ports.bootstrap.dependency_graph(),
                    &self.ports.bootstrap.component_states(),
                    &self.ui.collapsed_layers,
                )
                .len();
                if total_lines == 0 {
                    return;
                }
                let new_index = (self.ui.tree_selected as i32 + delta)
                    .clamp(0, (total_lines - 1) as i32) as usize;
                self.ui.tree_selected = new_index;
            }
            FocusTarget::Status => {
                let total = self.ports.bootstrap.dependency_graph().steps.len();
                if total == 0 {
                    return;
                }
                let new_index =
                    (self.ui.status_selected as i32 + delta).clamp(0, (total - 1) as i32) as usize;
                self.ui.status_selected = new_index;
            }
        }
    }

    fn scroll(&mut self, delta: i32) {
        let scroll = if delta.is_negative() {
            delta.unsigned_abs() as usize
        } else {
            delta as usize
        };

        match self.ui.focus {
            FocusTarget::Tree => {
                if delta.is_negative() {
                    self.ui.tree_scroll = self.ui.tree_scroll.saturating_sub(scroll);
                } else {
                    self.ui.tree_scroll = self.ui.tree_scroll.saturating_add(scroll);
                }
            }
            FocusTarget::Status => {
                if delta.is_negative() {
                    self.ui.status_scroll = self.ui.status_scroll.saturating_sub(scroll);
                } else {
                    self.ui.status_scroll = self.ui.status_scroll.saturating_add(scroll);
                }
            }
        }
    }

    fn toggle_expand_selected(&mut self) {
        if let Some(component) = self.selected_component_id() {
            if self.ui.expanded_components.contains(&component) {
                self.ui.expanded_components.remove(&component);
            } else {
                self.ui.expanded_components.insert(component);
            }
        }
    }

    fn toggle_layer_collapse(&mut self) {
        if self.ui.focus != FocusTarget::Tree {
            return;
        }
        let lines = build_tree_lines(
            self.ports.bootstrap.dependency_graph(),
            &self.ports.bootstrap.component_states(),
            &self.ui.collapsed_layers,
        );
        if let Some(TreeLine::Layer { layer, .. }) = lines.get(self.ui.tree_selected) {
            if self.ui.collapsed_layers.contains(layer) {
                self.ui.collapsed_layers.remove(layer);
            } else {
                self.ui.collapsed_layers.insert(*layer);
            }
        }
    }

    fn refresh_logs(&mut self) {
        let new_events = self.ports.logs.drain_events();
        if new_events.is_empty() {
            return;
        }

        let at_bottom = self.ui.log_scroll == 0;
        if !at_bottom {
            self.ui.log_scroll = self.ui.log_scroll.saturating_add(new_events.len());
        }

        for event in new_events {
            self.ui.log_events.push_back(event);
        }
        while self.ui.log_events.len() > LOG_MAX_EVENTS {
            self.ui.log_events.pop_front();
        }

        if at_bottom {
            self.ui.log_scroll = 0;
        } else {
            self.ui.log_scroll = self.ui.log_scroll.min(self.max_log_offset());
        }
    }

    fn handle_logs_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.ui.show_logs = false;
            }
            KeyCode::Up => self.scroll_logs(1),
            KeyCode::Down => self.scroll_logs(-1),
            KeyCode::PageUp => self.scroll_logs(5),
            KeyCode::PageDown => self.scroll_logs(-5),
            KeyCode::End => self.ui.log_scroll = 0,
            _ => {}
        }
        Ok(())
    }

    fn scroll_logs(&mut self, delta: i32) {
        let max_offset = self.max_log_offset();
        if delta.is_positive() {
            self.ui.log_scroll = self.ui.log_scroll.saturating_add(delta as usize);
        } else if delta.is_negative() {
            self.ui.log_scroll = self
                .ui
                .log_scroll
                .saturating_sub(delta.unsigned_abs() as usize);
        }
        self.ui.log_scroll = self.ui.log_scroll.min(max_offset);
    }

    fn max_log_offset(&self) -> usize {
        let view_height = self.ui.log_view_height.max(1);
        self.ui.log_events.len().saturating_sub(view_height)
    }
}
