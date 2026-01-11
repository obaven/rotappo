use crate::bootstrap::state::BootstrapUiState;
use bootstrappo::application::flows::reconcile::visualize::LayerType;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use rotappo_ports::{ComponentState, ComponentStatus, PortSet};
use std::time::Duration;

pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;
    if minutes > 0 {
        format!("{minutes}m {seconds}s")
    } else {
        format!("{seconds}s")
    }
}

pub fn progress_bar(progress: f32, width: usize) -> String {
    let filled = ((progress.clamp(0.0, 1.0) * width as f32).round() as usize).min(width);
    let mut bar = String::new();
    for i in 0..width {
        bar.push(if i < filled { '#' } else { '.' });
    }
    bar
}

pub fn table_widths(total_width: u16) -> [usize; 4] {
    let total = total_width.max(20) as usize;
    let component = total * 30 / 100;
    let status = total * 30 / 100;
    let time = total * 15 / 100;
    let progress = total - component - status - time;
    [
        component.max(12),
        status.max(12),
        time.max(6),
        progress.max(8),
    ]
}

pub fn format_row(values: &[impl AsRef<str>], widths: &[usize; 4]) -> String {
    let mut out = String::new();
    for (idx, value) in values.iter().enumerate() {
        let width = widths.get(idx).copied().unwrap_or(10);
        let text = value.as_ref();
        out.push_str(&format!("{text:<width$}"));
    }
    out
}

pub fn style_line(line: String, selected: bool) -> Line<'static> {
    if selected {
        Line::styled(line, Style::default().bg(Color::Blue).fg(Color::White))
    } else {
        Line::from(line)
    }
}

pub fn slice_lines(lines: &[Line<'static>], start: usize, height: usize) -> Vec<Line<'static>> {
    if lines.is_empty() {
        return Vec::new();
    }
    let start = start.min(lines.len().saturating_sub(1));
    let end = (start + height).min(lines.len());
    lines[start..end].to_vec()
}

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn status_icon(status: ComponentStatus) -> &'static str {
    match status {
        ComponentStatus::Pending => "PEND",
        ComponentStatus::Running => "RUN",
        ComponentStatus::Complete => "OK",
        ComponentStatus::Failed => "FAIL",
        ComponentStatus::Deferred => "DEF",
    }
}

pub fn layer_label(layer: LayerType) -> &'static str {
    match layer {
        LayerType::Network => "Network & Connectivity",
        LayerType::Storage => "Storage",
        LayerType::Security => "Security",
        LayerType::System => "System",
        LayerType::Datastores => "Datastores",
        LayerType::Observability => "Observability",
        LayerType::Analytics => "Analytics",
        LayerType::Entertainment => "Entertainment",
        LayerType::Infrastructure => "Infrastructure",
        LayerType::GitOps => "GitOps",
        LayerType::Unknown => "Other",
    }
}

pub fn format_status(state: &ComponentState) -> String {
    match state.status {
        ComponentStatus::Pending => "PEND Pending".to_string(),
        ComponentStatus::Running => {
            let phase = state
                .readiness
                .as_ref()
                .map(|r| r.basic.summary.clone())
                .unwrap_or_else(|| "Running".to_string());
            format!("RUN {phase}")
        }
        ComponentStatus::Complete => "OK Complete".to_string(),
        ComponentStatus::Failed => {
            let reason = state
                .deferred_reason
                .clone()
                .unwrap_or_else(|| "Failed".to_string());
            format!("FAIL {reason}")
        }
        ComponentStatus::Deferred => {
            let reason = state
                .deferred_reason
                .clone()
                .unwrap_or_else(|| "Deferred".to_string());
            format!("DEF {reason}")
        }
    }
}

pub fn find_dependents(
    assembly: &bootstrappo::domain::models::assembly::Assembly,
    target: &str,
) -> Vec<String> {
    assembly
        .steps
        .iter()
        .filter(|step| step.required.iter().any(|dep| dep == target))
        .map(|step| step.id.clone())
        .collect()
}

pub fn selected_component_label(ports: &PortSet, ui: &BootstrapUiState) -> Option<String> {
    ports
        .bootstrap
        .dependency_graph()
        .steps
        .get(ui.status_selected)
        .map(|step| step.id.clone())
}
