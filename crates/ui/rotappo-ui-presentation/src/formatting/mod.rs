//! Shared formatting helpers used by UI and CLI.

mod action;
mod problems;

pub use action::{action_groups, ActionGroup, ActionStepInfo};
pub use problems::problem_lines;
