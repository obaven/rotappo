//! Domain plan definitions used by the application layer.

#[derive(Debug, Clone)]
pub struct Plan {
    pub steps: Vec<PlanStepDef>,
}

#[derive(Debug, Clone)]
pub struct PlanStepDef {
    pub id: String,
    pub kind: String,
    pub depends_on: Vec<String>,
    pub provides: Vec<String>,
    pub domain: String,
    pub pod: Option<String>,
    pub has_gates: bool,
}
