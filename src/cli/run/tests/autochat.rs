use super::super::{AUTOCHAT_QUICK_DEMO_TASK, command_with_optional_args, resolve_autochat_model};

#[test]
fn parse_autochat_args_supports_default_count() {
    let parsed =
        crate::autochat::parse_autochat_request("build a relay", 3, AUTOCHAT_QUICK_DEMO_TASK)
            .expect("valid args");
    assert_eq!(
        (parsed.agent_count, parsed.task.as_str()),
        (3, "build a relay"),
    );
}

#[test]
fn parse_autochat_args_supports_explicit_count() {
    let parsed =
        crate::autochat::parse_autochat_request("4 build a relay", 3, AUTOCHAT_QUICK_DEMO_TASK)
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

#[test]
fn easy_go_defaults_to_minimax_when_model_not_set() {
    assert_eq!(
        resolve_autochat_model(None, None, Some("zai/glm-5"), true),
        "minimax/MiniMax-M3"
    );
}

#[test]
fn explicit_model_wins_over_easy_go_default() {
    assert_eq!(
        resolve_autochat_model(Some("zai/glm-5"), None, None, true),
        "zai/glm-5"
    );
}
