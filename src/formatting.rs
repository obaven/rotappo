use std::collections::BTreeMap;

use serde::Serialize;

use crate::adapters::bootstrappo::{mapping, LiveStatus};
use crate::runtime::{PlanStep, PlanStepStatus, Snapshot};

#[derive(Debug, Clone, Serialize)]
pub struct PlanStepInfo {
    pub index: usize,
    pub step: PlanStep,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanGroup {
    pub domain: String,
    pub steps: Vec<PlanStepInfo>,
}

pub fn plan_groups(snapshot: &Snapshot) -> Vec<PlanGroup> {
    let mut grouped: BTreeMap<String, Vec<PlanStepInfo>> = BTreeMap::new();
    for (index, step) in snapshot.plan_steps.iter().cloned().enumerate() {
        grouped
            .entry(step.domain.clone())
            .or_default()
            .push(PlanStepInfo { index, step });
    }

    grouped
        .into_iter()
        .map(|(domain, steps)| PlanGroup { domain, steps })
        .collect()
}

pub fn problem_lines(snapshot: &Snapshot, live_status: Option<&LiveStatus>) -> Vec<String> {
    let mut problems = Vec::new();
    if let Some(live) = live_status {
        if let Some(error) = live.last_error() {
            problems.push(format!("kube: {}", error));
        }
        let health = live.health();
        problems.extend(mapping::health_problem_lines(&health));
        if live.cache().is_none() {
            problems.push("kube cache not ready".to_string());
        }
    } else {
        problems.push("live status disabled".to_string());
    }

    for step in &snapshot.plan_steps {
        if step.status == PlanStepStatus::Blocked {
            problems.push(format!("blocked: {} waiting on {:?}", step.id, step.depends_on));
        }
    }

    problems
}
