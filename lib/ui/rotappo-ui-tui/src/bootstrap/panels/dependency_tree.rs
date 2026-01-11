use crate::bootstrap::state::{BootstrapUiState, FocusTarget};
use crate::bootstrap::utils::{format_duration, layer_label, status_icon};
use bootstrappo::application::flows::reconcile::visualize::LayerType;
use bootstrappo::application::flows::reconcile::visualize::layer::determine_layer;
use ratatui::layout::Rect;
use ratatui::prelude::Frame;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rotappo_ports::{ComponentState, ComponentStatus, PortSet};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub fn render(frame: &mut Frame, area: Rect, ports: &PortSet, ui: &BootstrapUiState) {
    let states = ports.bootstrap.component_states();
    let lines = build_tree_lines(
        ports.bootstrap.dependency_graph(),
        &states,
        &ui.collapsed_layers,
    );
    let start = ui.tree_scroll.min(lines.len().saturating_sub(1));
    let end = (start + area.height as usize).min(lines.len());

    let mut rendered = Vec::new();
    for (idx, line) in lines[start..end].iter().cloned().enumerate() {
        let line_index = start + idx;
        rendered.push(format_tree_line(
            line,
            ui.focus == FocusTarget::Tree && line_index == ui.tree_selected,
        ));
    }

    let block = Block::default()
        .title("Dependency Tree")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(rendered)
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

#[derive(Clone)]
pub enum TreeLine {
    Layer {
        layer: LayerType,
        total: usize,
        completed: usize,
    },
    Component {
        id: String,
        status: ComponentStatus,
        elapsed: Option<Duration>,
    },
}

pub fn build_tree_lines(
    assembly: &bootstrappo::domain::models::assembly::Assembly,
    states: &HashMap<String, ComponentState>,
    collapsed_layers: &HashSet<LayerType>,
) -> Vec<TreeLine> {
    let mut lines = Vec::new();
    for layer in ordered_layers() {
        let steps: Vec<_> = assembly
            .steps
            .iter()
            .filter(|step| determine_layer(step) == layer)
            .collect();
        if steps.is_empty() {
            continue;
        }
        let total = steps.len();
        let completed = steps
            .iter()
            .filter(|step| {
                states
                    .get(&step.id)
                    .map(|state| state.status == ComponentStatus::Complete)
                    .unwrap_or(false)
            })
            .count();

        lines.push(TreeLine::Layer {
            layer,
            total,
            completed,
        });

        if collapsed_layers.contains(&layer) {
            continue;
        }

        for step in steps {
            let status = states
                .get(&step.id)
                .map(|s| s.status)
                .unwrap_or(ComponentStatus::Pending);
            let elapsed = states
                .get(&step.id)
                .and_then(|s| s.timing.current_elapsed());
            lines.push(TreeLine::Component {
                id: step.id.clone(),
                status,
                elapsed,
            });
        }
    }

    lines
}

fn ordered_layers() -> Vec<LayerType> {
    vec![
        LayerType::Network,
        LayerType::Storage,
        LayerType::Security,
        LayerType::System,
        LayerType::Datastores,
        LayerType::Observability,
        LayerType::Analytics,
        LayerType::Entertainment,
        LayerType::Infrastructure,
        LayerType::GitOps,
        LayerType::Unknown,
    ]
}

fn format_tree_line(line: TreeLine, selected: bool) -> Line<'static> {
    let (text, style) = match line {
        TreeLine::Layer {
            layer,
            total,
            completed,
        } => {
            let status_tag = if completed == total { "OK" } else { "PEND" };
            let label = format!(
                "{} [{} {}/{}]",
                layer_label(layer),
                status_tag,
                completed,
                total
            );
            (label, Style::default().fg(Color::Yellow))
        }
        TreeLine::Component {
            id,
            status,
            elapsed,
        } => {
            let icon = status_icon(status);
            let elapsed_text = elapsed
                .map(format_duration)
                .unwrap_or_else(|| "-".to_string());
            let label = format!("  {icon} {id} ({elapsed_text})");
            let style = match status {
                ComponentStatus::Running => Style::default().fg(Color::Cyan),
                ComponentStatus::Failed => Style::default().fg(Color::Red),
                ComponentStatus::Deferred => Style::default().fg(Color::DarkGray),
                ComponentStatus::Complete => Style::default().fg(Color::Green),
                ComponentStatus::Pending => Style::default().fg(Color::White),
            };
            (label, style)
        }
    };

    let styled = if selected {
        Style::default().bg(Color::Blue).fg(Color::White)
    } else {
        style
    };
    Line::styled(text, styled)
}
