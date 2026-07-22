use super::validate;

fn call(name: &str, arguments: &str) -> (String, String) {
    (name.into(), arguments.into())
}

#[test]
fn permits_independent_read_batch() {
    let calls = vec![call("read", "{}"), call("grep", "{}")];
    assert!(validate(&calls).is_ok());
}

#[test]
fn rejects_patch_followed_by_premature_validation() {
    let calls = vec![call("apply_patch", "{}"), call("exec_command", "{}")];
    assert!(
        validate(&calls)
            .unwrap_err()
            .to_string()
            .contains("apply_patch")
    );
}

#[test]
fn rejects_goal_completion_batched_before_command_result() {
    let calls = vec![
        call("exec_command", "{}"),
        call("update_goal", r#"{"status":"complete"}"#),
    ];
    assert!(
        validate(&calls)
            .unwrap_err()
            .to_string()
            .contains("only call")
    );
}
