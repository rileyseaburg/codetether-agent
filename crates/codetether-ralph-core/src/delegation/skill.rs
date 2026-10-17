//! skill policy for a Ralph relay role.
use super::*;

pub(super) fn skill_for_role(role: &str) -> &'static str {
    match role {
        "auto-coder" => ROLE_CODER,
        "auto-reviewer" => ROLE_REVIEWER,
        "auto-tester" => ROLE_TESTER,
        _ => delegation_skills::MODEL_CALL,
    }
}
