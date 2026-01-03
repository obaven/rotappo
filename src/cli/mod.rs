use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::formatting;
use crate::runtime::{Action, Event, Snapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Plain,
    Json,
    Ndjson,
}

impl OutputMode {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "plain" => Some(OutputMode::Plain),
            "json" => Some(OutputMode::Json),
            "ndjson" => Some(OutputMode::Ndjson),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            OutputMode::Plain => "plain",
            OutputMode::Json => "json",
            OutputMode::Ndjson => "ndjson",
        }
    }
}

pub fn format_actions(mode: OutputMode, actions: &[Action]) -> Result<String> {
    match mode {
        OutputMode::Plain => Ok(actions
            .iter()
            .map(|action| {
                format!(
                    "{} - {} ({})",
                    action.id,
                    action.label,
                    action.safety.as_str()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")),
        OutputMode::Json => Ok(serde_json::to_string_pretty(actions)?),
        OutputMode::Ndjson => to_ndjson(actions),
    }
}

pub fn format_snapshot(mode: OutputMode, snapshot: &Snapshot) -> Result<String> {
    match mode {
        OutputMode::Plain => Ok(format!(
            "Plan {}/{} complete | Health: {}",
            snapshot.plan.completed,
            snapshot.plan.total,
            snapshot.health.as_str()
        )),
        OutputMode::Json => Ok(serde_json::to_string_pretty(snapshot)?),
        OutputMode::Ndjson => to_ndjson(snapshot),
    }
}

pub fn format_events(mode: OutputMode, events: &[Event]) -> Result<String> {
    match mode {
        OutputMode::Plain => Ok(events
            .iter()
            .map(|event| {
                format!(
                    "[{}] {} {}",
                    event.level.as_str(),
                    event.timestamp_ms,
                    event.message
                )
            })
            .collect::<Vec<_>>()
            .join("\n")),
        OutputMode::Json => Ok(serde_json::to_string_pretty(events)?),
        OutputMode::Ndjson => to_ndjson(events),
    }
}

pub fn format_plan(mode: OutputMode, snapshot: &Snapshot) -> Result<String> {
    let groups = formatting::plan_groups(snapshot);
    match mode {
        OutputMode::Plain => {
            let mut lines = Vec::new();
            for group in groups {
                lines.push(format!("{} domain", group.domain));
                for step_info in group.steps {
                    let step = step_info.step;
                    let pod_text = step
                        .pod
                        .as_deref()
                        .map(|pod| format!(" pod: {}", pod))
                        .unwrap_or_else(|| " pod: -".to_string());
                    lines.push(format!(
                        "[{:<9}] {} {}{}",
                        step.status.as_str(),
                        step.id,
                        step.kind,
                        pod_text
                    ));
                }
            }
            Ok(lines.join("\n"))
        }
        OutputMode::Json => Ok(serde_json::to_string_pretty(&groups)?),
        OutputMode::Ndjson => to_ndjson(&groups),
    }
}

pub fn format_problems(mode: OutputMode, problems: &[String]) -> Result<String> {
    match mode {
        OutputMode::Plain => Ok(problems.join("\n")),
        OutputMode::Json => Ok(serde_json::to_string_pretty(problems)?),
        OutputMode::Ndjson => to_ndjson(problems),
    }
}

fn to_ndjson<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    let json = serde_json::to_string(value)?;
    if json.contains('\n') {
        return Err(anyhow!("NDJSON payload contains newlines"));
    }
    Ok(json)
}
