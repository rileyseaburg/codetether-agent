use super::super::{
    AUTOCHAT_QUICK_DEMO_TASK, is_easy_go_command, normalize_cli_go_command,
    normalize_go_task_input, validate_easy_go_task,
};

#[test]
fn normalize_go_maps_to_autochat_with_count_and_task() {
    assert_eq!(
        normalize_cli_go_command("/go 4 build protocol relay"),
        "/autochat 4 build protocol relay"
    );
}

#[test]
fn normalize_go_count_only_uses_demo_task() {
    assert_eq!(
        normalize_cli_go_command("/go 4"),
        format!("/autochat 4 {AUTOCHAT_QUICK_DEMO_TASK}")
    );
}

#[test]
fn normalize_go_task_collapses_whitespace() {
    assert_eq!(
        normalize_go_task_input(" implement   api\ncompat routes\twith tests "),
        "implement api compat routes with tests"
    );
}

#[test]
fn validate_go_task_rejects_pasted_run_output() {
    let pasted =
        "Task: foo Progress: 0/7 stories Iterations: 7/10 Incomplete stories: ... Next steps:";
    assert!(validate_easy_go_task(pasted).is_err());
}

#[test]
fn easy_go_detection_handles_aliases() {
    assert!(is_easy_go_command("/go 4 task"));
    assert!(is_easy_go_command("/team 4 task"));
    assert!(!is_easy_go_command("/autochat 4 task"));
}
