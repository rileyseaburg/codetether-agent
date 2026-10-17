//! Existing auto-chat parsing and policy regression.
use super::*;

#[test]
fn parse_autochat_args_with_count_only_uses_demo_task() {
    let parsed = parse_autochat_args("4", AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_QUICK_DEMO_TASK);
    assert_eq!(parsed, Some((4, AUTOCHAT_QUICK_DEMO_TASK)));
}
