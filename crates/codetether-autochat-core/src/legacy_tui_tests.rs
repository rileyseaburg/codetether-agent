//! Existing TUI regression cases moved beside their implementation.
use crate::AUTOCHAT_QUICK_DEMO_TASK;

#[test]
fn parse_autochat_args_supports_default_count() {
    let parsed = crate::parse_autochat_request("build a calculator", 3, AUTOCHAT_QUICK_DEMO_TASK);
    assert_eq!(
        parsed.map(|value| (value.agent_count, value.task)),
        Some((3, "build a calculator".to_string()))
    );
}

#[test]
fn parse_autochat_args_supports_explicit_count() {
    let parsed = crate::parse_autochat_request("4 build a calculator", 3, AUTOCHAT_QUICK_DEMO_TASK);
    assert_eq!(
        parsed.map(|value| (value.agent_count, value.task)),
        Some((4, "build a calculator".to_string()))
    );
}

#[test]
fn parse_autochat_args_count_only_uses_quick_demo_task() {
    let parsed = crate::parse_autochat_request("4", 3, AUTOCHAT_QUICK_DEMO_TASK);
    assert_eq!(
        parsed.map(|value| (value.agent_count, value.task)),
        Some((4, AUTOCHAT_QUICK_DEMO_TASK.to_string()))
    );
}
