//! Plan rendering helpers.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use rotappo_ui_presentation::formatting;
use rotappo_domain::{CapabilityStatus, PlanStepStatus};

use super::spinner_frame;

/// Rendered line for a plan view.
pub struct PlanLine {
    pub line: Line<'static>,
    pub step_index: Option<usize>,
}

/// Icon representing a plan step status.
///
/// # Examples
/// ```rust
/// use rotappo_domain::PlanStepStatus;
/// use rotappo_ui_tui::util::plan_status_icon;
///
/// assert_eq!(plan_status_icon(PlanStepStatus::Succeeded), '+');
/// ```
pub fn plan_status_icon(status: PlanStepStatus) -> char {
    match status {
        PlanStepStatus::Running => spinner_frame(),
        PlanStepStatus::Succeeded => '+',
        PlanStepStatus::Failed => 'x',
        PlanStepStatus::Blocked => '!',
        PlanStepStatus::Pending => '.',
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

/// Build the formatted plan lines for display.
pub fn plan_lines(snapshot: &rotappo_domain::Snapshot) -> Vec<PlanLine> {
    let mut lines = Vec::new();
    for group in formatting::plan_groups(snapshot) {
        lines.push(PlanLine {
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
                PlanStepStatus::Succeeded => Style::default().fg(Color::Green),
                PlanStepStatus::Running => Style::default().fg(Color::Yellow),
                PlanStepStatus::Blocked => Style::default().fg(Color::Red),
                PlanStepStatus::Failed => {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                }
                PlanStepStatus::Pending => Style::default().fg(Color::Gray),
            };
            let pod_text = step
                .pod
                .as_deref()
                .map(|pod| format!(" pod: {}", pod))
                .unwrap_or_else(|| " pod: -".to_string());
            let line = Line::from(vec![
                Span::styled(
                    format!("[{} {:<9}]", plan_status_icon(step.status), step.status.as_str()),
                    status_style,
                ),
                Span::raw(" "),
                Span::styled(step.id.clone(), Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::raw(step.kind.clone()),
                Span::styled(pod_text, Style::default().fg(Color::DarkGray)),
            ]);
            lines.push(PlanLine {
                line,
                step_index: Some(step_info.index),
            });
        }
    }
    lines
}
