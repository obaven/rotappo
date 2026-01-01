use ratatui::layout::Rect;

use super::{App, PanelId, COLLAPSED_HEIGHT};

impl App {
    pub fn handle_header_click(&mut self, column: u16, row: u16) -> bool {
        if self.ui.settings_gear_area.contains((column, row).into()) {
            self.toggle_settings_panel();
            return true;
        }
        self.toggle_if_header(PanelId::PlanProgress, self.ui.plan_progress_area, column, row)
            || self.toggle_if_header(PanelId::Snapshot, self.ui.snapshot_area, column, row)
            || self.toggle_if_header(PanelId::Capabilities, self.ui.capabilities_area, column, row)
            || self.toggle_if_header(PanelId::PlanSteps, self.ui.plan_area, column, row)
            || self.toggle_if_header(PanelId::Actions, self.ui.actions_area, column, row)
            || self.toggle_if_header(PanelId::Settings, self.ui.settings_area, column, row)
            || self.toggle_if_header(PanelId::LogControls, self.ui.log_controls_area, column, row)
            || self.toggle_if_header(PanelId::Problems, self.ui.problems_area, column, row)
            || self.toggle_if_header(PanelId::Logs, self.ui.logs_area, column, row)
            || self.toggle_if_header(PanelId::Help, self.ui.help_area, column, row)
    }

    fn toggle_if_header(
        &mut self,
        panel: PanelId,
        area: Rect,
        column: u16,
        row: u16,
    ) -> bool {
        if area.height < 1 {
            return false;
        }
        if area.contains((column, row).into()) && row == area.y {
            self.toggle_panel(panel);
            return true;
        }
        false
    }

    fn toggle_panel(&mut self, panel: PanelId) {
        let currently_collapsed = self.is_collapsed(panel);
        self.set_collapsed(panel, !currently_collapsed);
        if currently_collapsed {
            self.ensure_space_for(panel);
        }
    }

    pub fn toggle_settings_panel(&mut self) {
        let currently_collapsed = self.ui.collapsed_settings;
        self.set_collapsed(PanelId::Settings, !currently_collapsed);
        if currently_collapsed {
            self.ensure_space_for(PanelId::Settings);
        }
    }

    fn is_collapsed(&self, panel: PanelId) -> bool {
        match panel {
            PanelId::PlanProgress => self.ui.collapsed_plan_progress,
            PanelId::Snapshot => self.ui.collapsed_snapshot,
            PanelId::Capabilities => self.ui.collapsed_capabilities,
            PanelId::PlanSteps => self.ui.collapsed_plan_steps,
            PanelId::Actions => self.ui.collapsed_actions,
            PanelId::Settings => self.ui.collapsed_settings,
            PanelId::LogControls => self.ui.collapsed_log_controls,
            PanelId::Problems => self.ui.collapsed_problems,
            PanelId::Logs => self.ui.collapsed_logs,
            PanelId::Help => self.ui.collapsed_help,
        }
    }

    fn set_collapsed(&mut self, panel: PanelId, value: bool) {
        match panel {
            PanelId::PlanProgress => self.ui.collapsed_plan_progress = value,
            PanelId::Snapshot => self.ui.collapsed_snapshot = value,
            PanelId::Capabilities => self.ui.collapsed_capabilities = value,
            PanelId::PlanSteps => self.ui.collapsed_plan_steps = value,
            PanelId::Actions => self.ui.collapsed_actions = value,
            PanelId::Settings => self.ui.collapsed_settings = value,
            PanelId::LogControls => self.ui.collapsed_log_controls = value,
            PanelId::Problems => self.ui.collapsed_problems = value,
            PanelId::Logs => self.ui.collapsed_logs = value,
            PanelId::Help => self.ui.collapsed_help = value,
        }
        if !value {
            if matches!(panel, PanelId::Help | PanelId::Logs) {
                self.ui.collapsed_plan_steps = true;
            }
            self.mark_opened(panel);
        }
    }

    fn desired_height(panel: PanelId) -> u16 {
        match panel {
            PanelId::PlanProgress => 3,
            PanelId::Snapshot => 6,
            PanelId::Capabilities => 6,
            PanelId::PlanSteps => 6,
            PanelId::Actions => 8,
            PanelId::Settings => 6,
            PanelId::LogControls => 3,
            PanelId::Problems => 4,
            PanelId::Logs => 8,
            PanelId::Help => 2,
        }
    }

    fn ensure_space_for(&mut self, panel: PanelId) {
        match panel {
            PanelId::PlanProgress | PanelId::Snapshot | PanelId::Capabilities => {
                let column_height = self.ui.plan_progress_area.height
                    + self.ui.snapshot_area.height
                    + self.ui.capabilities_area.height;
                let panels = [PanelId::PlanProgress, PanelId::Snapshot, PanelId::Capabilities];
                self.collapse_if_needed(panel, column_height, &panels);
            }
            PanelId::PlanSteps => {}
            PanelId::Actions | PanelId::Problems => {
                let column_height =
                    self.ui.actions_area.height + self.ui.problems_area.height;
                let panels = [PanelId::Actions, PanelId::Problems];
                self.collapse_if_needed(panel, column_height, &panels);
            }
            PanelId::LogControls | PanelId::Logs => {
                let column_height =
                    self.ui.log_controls_area.height + self.ui.logs_area.height;
                let panels = [PanelId::LogControls, PanelId::Logs];
                self.collapse_if_needed(panel, column_height, &panels);
            }
            PanelId::Settings => {}
            PanelId::Help => {
                if self.ui.screen_area.height < 20 {
                    self.ui.collapsed_plan_progress = true;
                    self.ui.collapsed_snapshot = true;
                    self.ui.collapsed_capabilities = true;
                    self.ui.collapsed_plan_steps = true;
                    self.ui.collapsed_actions = true;
                    self.ui.collapsed_settings = true;
                    self.ui.collapsed_log_controls = true;
                    self.ui.collapsed_problems = true;
                    self.ui.collapsed_logs = true;
                }
            }
        }
    }

    fn collapse_if_needed(&mut self, target: PanelId, column_height: u16, panels: &[PanelId]) {
        let mut needed = 0u16;
        for panel in panels {
            if *panel == target {
                needed = needed.saturating_add(Self::desired_height(*panel));
            } else if self.is_collapsed(*panel) {
                needed = needed.saturating_add(COLLAPSED_HEIGHT);
            } else {
                needed = needed.saturating_add(Self::desired_height(*panel));
            }
        }
        if needed > column_height {
            for panel in panels {
                if *panel != target {
                    self.set_collapsed(*panel, true);
                }
            }
        }
    }

    fn mark_opened(&mut self, panel: PanelId) {
        self.ui.open_counter = self.ui.open_counter.wrapping_add(1);
        match panel {
            PanelId::Help => self.ui.help_opened_at = Some(self.ui.open_counter),
            PanelId::Logs => self.ui.logs_opened_at = Some(self.ui.open_counter),
            _ => {}
        }
    }

    pub fn middle_aux_panel(&self) -> Option<PanelId> {
        if !self.ui.collapsed_plan_steps {
            return None;
        }
        let help_open = !self.ui.collapsed_help;
        let logs_open = !self.ui.collapsed_logs;
        match (help_open, logs_open) {
            (true, false) => Some(PanelId::Help),
            (false, true) => Some(PanelId::Logs),
            (true, true) => {
                let help_at = self.ui.help_opened_at.unwrap_or(u64::MAX);
                let logs_at = self.ui.logs_opened_at.unwrap_or(u64::MAX);
                if help_at <= logs_at {
                    Some(PanelId::Help)
                } else {
                    Some(PanelId::Logs)
                }
            }
            _ => None,
        }
    }
}
