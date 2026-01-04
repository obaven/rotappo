use std::collections::BTreeMap;

use serde::Serialize;

use rotappo_domain::{PlanStep, Snapshot};

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
