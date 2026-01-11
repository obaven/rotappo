//! Settings panel view model.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

#[derive(Clone)]
pub(super) struct SettingsView {
    pub(super) host: String,
    pub(super) assembly_path: String,
    pub(super) config_path: String,
    pub(super) log_filter: String,
    pub(super) log_interval: u64,
    pub(super) log_paused: bool,
    pub(super) auto_refresh: bool,
    pub(super) selected: usize,
}

pub(super) fn settings_lines(view: &SettingsView) -> Vec<Line> {
    let apply_style = if view.selected == 0 {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default().fg(Color::Cyan)
    };
    let cancel_style = if view.selected == 1 {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default().fg(Color::Cyan)
    };
    vec![
        Line::from(Span::styled(
            "Runtime",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("Host: {host}", host = view.host.as_str())),
        Line::from(format!(
            "Assembly: {assembly}",
            assembly = view.assembly_path.as_str()
        )),
        Line::from(format!(
            "Config: {config}",
            config = view.config_path.as_str()
        )),
        Line::from(""),
        Line::from(Span::styled(
            "UI",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(format!(
            "Log filter: {filter} (press f to cycle)",
            filter = view.log_filter.as_str()
        )),
        Line::from(format!(
            "Stream interval: {interval}s",
            interval = view.log_interval
        )),
        Line::from(format!(
            "Stream paused: {paused} (hold p to toggle)",
            paused = view.log_paused
        )),
        Line::from(format!(
            "Watch mode: {mode} (press w to toggle)",
            mode = if view.auto_refresh { "on" } else { "off" }
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Controls",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("[Apply]", apply_style),
            Span::raw("  "),
            Span::styled("[Cancel]", cancel_style),
        ]),
    ]
}
