//! Action rendering helpers.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use rotappo_ui_presentation::formatting;
use rotappo_domain::{CapabilityStatus, ActionStepStatus};

use super::spinner_frame;

/// Rendered line for a action view.
pub struct ActionLine {
    pub line: Line<'static>,
    pub step_index: Option<usize>,
}

/// Icon representing a action step status.
///
/// # Examples
/// ```rust
/// use rotappo_domain::ActionStepStatus;
/// use rotappo_ui_tui::util::action_status_icon;
///
/// assert_eq!(action_status_icon(ActionStepStatus::Succeeded), '+');
/// ```
pub fn action_status_icon(status: ActionStepStatus) -> char {
    match status {
        ActionStepStatus::Running => spinner_frame(),
        ActionStepStatus::Succeeded => '+',
        ActionStepStatus::Failed => 'x',
        ActionStepStatus::Blocked => '!',
        ActionStepStatus::Pending => '.',
    }
}

/// Icon representing a capability status.
///
/// # Examples
/// ```rust
/// use rotappo_domain::CapabilityStatus;
/// use rotappo_ui_tui::util::capability_icon;
///
/// assert_eq!(capability_icon(CapabilityStatus::Ready), '+');
/// ```
pub fn capability_icon(status: CapabilityStatus) -> char {
    match status {
        CapabilityStatus::Ready => '+',
        CapabilityStatus::Degraded => '!',
        CapabilityStatus::Offline => 'x',
    }
}

/// Build the formatted action lines for display.
pub fn action_lines(snapshot: &rotappo_domain::Snapshot) -> Vec<ActionLine> {
    let mut lines = Vec::new();
    for group in formatting::action_groups(snapshot) {
        lines.push(ActionLine {
            line: Line::from(Span::styled(
                format!("{} domain", group.domain),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            step_index: None,
        });
        for step_info in group.steps {
            let step = &step_info.step;
            let status_style = match step.status {
                ActionStepStatus::Succeeded => Style::default().fg(Color::Green),
                ActionStepStatus::Running => Style::default().fg(Color::Yellow),
                ActionStepStatus::Blocked => Style::default().fg(Color::Red),
                ActionStepStatus::Failed => {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                }
                ActionStepStatus::Pending => Style::default().fg(Color::Gray),
            };
            let pod_text = step
                .pod
                .as_deref()
                .map(|pod| format!(" pod: {}", pod))
                .unwrap_or_else(|| " pod: -".to_string());
            let line = Line::from(vec![
                Span::styled(
                    format!("[{} {:<9}]", action_status_icon(step.status), step.status.as_str()),
                    status_style,
                ),
                Span::raw(" "),
                Span::styled(step.id.clone(), Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::raw(step.kind.clone()),
                Span::styled(pod_text, Style::default().fg(Color::DarkGray)),
            ]);
            lines.push(ActionLine {
                line,
                step_index: Some(step_info.index),
            });
        }
    }
    lines
}
