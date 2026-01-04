//! Shared formatting helpers used by UI and CLI.

mod plan;
mod problems;

pub use plan::{plan_groups, PlanGroup, PlanStepInfo};
pub use problems::problem_lines;
