//! Existing auto-chat parsing and policy regression.
use super::*;

#[test]
fn parse_autochat_args_with_default_count() {
    let parsed = parse_autochat_args(
        "implement relay checkpointing",
        AUTOCHAT_DEFAULT_AGENTS,
        AUTOCHAT_QUICK_DEMO_TASK,
    );
    assert_eq!(
        parsed,
        Some((AUTOCHAT_DEFAULT_AGENTS, "implement relay checkpointing"))
    );
}
