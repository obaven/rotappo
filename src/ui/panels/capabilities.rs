use ratatui::{
    layout::{Margin, Rect},
    prelude::Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
};

use crate::ui::app::App;
use crate::ui::state::HoverPanel;
use crate::ui::util::{capability_icon, traveling_glow};

pub fn render_capabilities(frame: &mut Frame, area: Rect, app: &mut App) {
    app.ui.capabilities_area = area;
    if app.ui.collapsed_capabilities {
        let mut block = Block::default().title("Capabilities").borders(Borders::ALL);
        if app.ui.hover_panel == HoverPanel::Capabilities {
            block = block.style(Style::default().bg(Color::Rgb(0, 90, 90)));
        }
        frame.render_widget(block, area);
        return;
    }
    let snapshot = app.runtime.snapshot();
    let total_caps = snapshot.capabilities.len();
    let lines: Vec<Line> = snapshot
        .capabilities
        .iter()
        .skip(app.ui.capabilities_scroll as usize)
        .enumerate()
        .map(|(offset, capability)| {
            let status_style = match capability.status {
                crate::runtime::CapabilityStatus::Ready => Style::default().fg(Color::Green),
                crate::runtime::CapabilityStatus::Degraded => Style::default().fg(Color::Yellow),
                crate::runtime::CapabilityStatus::Offline => Style::default().fg(Color::Red),
            };
            let line_index = app.ui.capabilities_scroll as usize + offset;
            let mut line_style = Style::default();
            if let Some(color) = traveling_glow(line_index, total_caps) {
                line_style = line_style.fg(color);
            }
            Line::from(vec![
                Span::styled(
                    format!(
                        "[{} {:<8}]",
                        capability_icon(capability.status),
                        capability.status.as_str()
                    ),
                    status_style,
                ),
                Span::raw(" "),
                Span::raw(&capability.name),
            ])
            .style(line_style)
        })
        .collect();

    let mut panel_block = Block::default().title("Capabilities").borders(Borders::ALL);
    if app.refresh_pulse_active() {
        panel_block = panel_block.style(Style::default().fg(Color::Cyan));
    }
    if app.ui.hover_panel == HoverPanel::Capabilities {
        let active_style = Style::default().bg(Color::Rgb(0, 90, 90));
        panel_block = panel_block.style(active_style);
    }
    let mut panel = Paragraph::new(lines)
        .block(panel_block)
        .wrap(Wrap { trim: true });
    if app.ui.hover_panel == HoverPanel::Capabilities {
        let active_style = Style::default().bg(Color::Rgb(0, 90, 90));
        panel = panel.style(active_style);
    }
    frame.render_widget(panel, area);

    let view_height = area.height.saturating_sub(2) as usize;
    if total_caps > view_height && view_height > 0 {
        let mut state =
            ScrollbarState::new(total_caps).position(app.ui.capabilities_scroll as usize);
        let bar =
            Scrollbar::new(ScrollbarOrientation::VerticalRight).style(Style::default().fg(Color::Cyan));
        frame.render_stateful_widget(
            bar,
            area.inner(Margin {
                horizontal: 0,
                vertical: 1,
            }),
            &mut state,
        );
    }
}
