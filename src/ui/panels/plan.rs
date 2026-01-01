use ratatui::{
    layout::Rect,
    prelude::Frame,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
};

use crate::ui::app::App;
use crate::ui::util::format_age;

pub fn render_plan(frame: &mut Frame, area: Rect, app: &mut App) {
    let snapshot = app.runtime.snapshot();
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(if app.ui.collapsed_plan_progress { 2 } else { 3 }),
            ratatui::layout::Constraint::Min(0),
        ])
        .split(area);
    app.ui.plan_progress_area = chunks[0];
    app.ui.snapshot_area = chunks[1];

    if app.ui.collapsed_plan_progress {
        let block = Block::default().title("Plan Progress").borders(Borders::ALL);
        frame.render_widget(block, chunks[0]);
    } else {
        let percent = snapshot.plan.percent_complete();
        let mut gauge_block = Block::default().title("Plan Progress").borders(Borders::ALL);
        if app.refresh_pulse_active() {
            gauge_block = gauge_block.style(Style::default().fg(Color::Cyan));
        }
        let gauge = Gauge::default()
            .block(gauge_block)
            .gauge_style(Style::default().fg(Color::Green))
            .percent(percent)
            .label(format!("{}%", percent));
        frame.render_widget(gauge, chunks[0]);
    }

    let age = format_age(snapshot.last_updated_ms);
    let lines = vec![
        Line::from(format!(
            "Complete: {}/{}",
            snapshot.plan.completed, snapshot.plan.total
        )),
        Line::from(format!("In progress: {}", snapshot.plan.in_progress)),
        Line::from(format!("Blocked: {}", snapshot.plan.blocked)),
        Line::from(format!("Pending: {}", snapshot.plan.pending)),
        Line::from(format!("Health: {}", snapshot.health.as_str())),
        Line::from(format!("Last update: {}", age)),
        Line::from(format!(
            "Last action: {}",
            snapshot
                .last_action
                .map(|action| action.to_string())
                .unwrap_or_else(|| "none".to_string())
        )),
    ];

    if app.ui.collapsed_snapshot {
        let mut block = Block::default().title("Snapshot").borders(Borders::ALL);
        if app.ui.hover_snapshot {
            let active_style = Style::default().bg(Color::Rgb(0, 90, 90));
            block = block.style(active_style);
        }
        frame.render_widget(block, chunks[1]);
    } else {
        let mut summary_block = Block::default().title("Snapshot").borders(Borders::ALL);
        if app.refresh_pulse_active() {
            summary_block = summary_block.style(Style::default().fg(Color::Cyan));
        }
        if app.ui.hover_snapshot {
            let active_style = Style::default().bg(Color::Rgb(0, 90, 90));
            summary_block = summary_block.style(active_style);
        }
        let mut summary = Paragraph::new(lines)
            .block(summary_block)
            .wrap(Wrap { trim: true });
        if app.ui.hover_snapshot {
            let active_style = Style::default().bg(Color::Rgb(0, 90, 90));
            summary = summary.style(active_style);
        }
        frame.render_widget(summary, chunks[1]);
    }
}
