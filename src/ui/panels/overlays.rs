use ratatui::{
    layout::Rect,
    prelude::Frame,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::ui::app::App;
use crate::ui::util::{centered_rect, tooltip_rect_for_mouse, tooltip_rect_in_corner};

pub fn render_tooltip(frame: &mut Frame, app: &mut App) {
    let pinned = app.ui.pinned_tooltip.clone();
    let hover = app.current_tooltip();
    let mut pinned_area = None;

    if let Some(pinned) = pinned {
        let lines = tooltip_lines(&pinned);
        let area = tooltip_rect_in_corner(frame.area(), &lines, 38, 30, 2, 1);
        frame.render_widget(Clear, area);
        render_tooltip_box(frame, area, lines);
        pinned_area = Some(area);
    }

    if let Some(hover) = hover {
        let lines = tooltip_lines(&hover);
        let mut area = tooltip_rect_for_mouse(frame.area(), app.ui.mouse_pos, &lines, 45, 35);
        if let Some(pinned_area) = pinned_area {
            if rects_overlap(area, pinned_area) {
                let alternate_pos = app
                    .ui
                    .mouse_pos
                    .map(|(x, y)| (x.saturating_sub(4), y.saturating_sub(2)));
                area = tooltip_rect_for_mouse(frame.area(), alternate_pos, &lines, 45, 35);
            }
        }
        frame.render_widget(Clear, area);
        render_tooltip_box(frame, area, lines);
    }
}

fn render_tooltip_box(frame: &mut Frame, area: Rect, lines: Vec<Line>) {
    let panel = Paragraph::new(lines)
        .block(Block::default().title("Tooltip").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(panel, area);
}

fn tooltip_lines(tooltip: &crate::ui::state::Tooltip) -> Vec<Line> {
    std::iter::once(Line::from(Span::styled(
        tooltip.title.clone(),
        Style::default().add_modifier(Modifier::BOLD),
    )))
    .chain(tooltip.lines.iter().cloned().map(Line::from))
    .collect()
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let a_right = a.x.saturating_add(a.width);
    let a_bottom = a.y.saturating_add(a.height);
    let b_right = b.x.saturating_add(b.width);
    let b_bottom = b.y.saturating_add(b.height);
    a.x < b_right && a_right > b.x && a.y < b_bottom && a_bottom > b.y
}

pub fn render_confirmation(frame: &mut Frame, app: &mut App) {
    let Some(confirm) = &app.confirm else {
        return;
    };

    let area = centered_rect(60, 30, frame.area());
    frame.render_widget(Clear, area);

    let lines = vec![
        Line::from(Span::styled(
            "Confirm Action",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("Action: {}", confirm.label)),
        Line::from(format!("Safety: {}", confirm.safety.as_str())),
        Line::from(""),
        Line::from("Press Y to confirm, N to cancel"),
    ];

    let panel = Paragraph::new(lines)
        .block(Block::default().title("Confirmation").borders(Borders::ALL))
        .alignment(ratatui::prelude::Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(panel, area);
}
