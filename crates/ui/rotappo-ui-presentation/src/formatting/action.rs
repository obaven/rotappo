use std::collections::BTreeMap;

use serde::Serialize;

use rotappo_domain::{ActionStep, Snapshot};

#[derive(Debug, Clone, Serialize)]
pub struct ActionStepInfo {
    pub index: usize,
    pub step: ActionStep,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActionGroup {
    pub domain: String,
    pub steps: Vec<ActionStepInfo>,
}

pub fn action_groups(snapshot: &Snapshot) -> Vec<ActionGroup> {
    let mut grouped: BTreeMap<String, Vec<ActionStepInfo>> = BTreeMap::new();
    for (index, step) in snapshot.action_steps.iter().cloned().enumerate() {
        grouped
            .entry(step.domain.clone())
            .or_default()
            .push(ActionStepInfo { index, step });
    }

    grouped
        .into_iter()
        .map(|(domain, steps)| ActionGroup { domain, steps })
        .collect()
}
