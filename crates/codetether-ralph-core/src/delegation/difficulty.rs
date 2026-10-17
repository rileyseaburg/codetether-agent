//! difficulty policy for a Ralph relay role.
use super::*;

pub(super) fn difficulty_for_role(role: &str) -> Difficulty {
    if role.contains("planner") || role.contains("reviewer") {
        Difficulty::Medium
    } else if role.contains("coder") {
        Difficulty::Hard
    } else {
        Difficulty::Easy
    }
}
