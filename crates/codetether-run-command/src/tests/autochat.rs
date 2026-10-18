//! Model precedence for CLI relay requests.
use crate::command_with_optional_args;
use codetether_autochat_core::AUTOCHAT_QUICK_DEMO_TASK;

#[test]
fn parse_autochat_args_supports_default_count() {
    let parsed = codetether_autochat_core::parse_autochat_request(
        "build a relay",
        3,
        AUTOCHAT_QUICK_DEMO_TASK,
    )
    .expect("valid args");
    assert_eq!(
        (parsed.agent_count, parsed.task.as_str()),
        (3, "build a relay"),
    );
}

#[test]
fn parse_autochat_args_supports_explicit_count() {
    let parsed = codetether_autochat_core::parse_autochat_request(
        "4 build a relay",
        3,
        AUTOCHAT_QUICK_DEMO_TASK,
    )
    .expect("valid args");
    assert_eq!(
        (parsed.agent_count, parsed.task.as_str()),
        (4, "build a relay"),
    );
}

#[test]
fn command_with_optional_args_avoids_prefix_collision() {
    assert_eq!(command_with_optional_args("/autochatty", "/autochat"), None);
}
