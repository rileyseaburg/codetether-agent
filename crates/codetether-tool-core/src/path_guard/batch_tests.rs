use super::normalize_tool_args;
use serde_json::json;

#[test]
fn normalizes_paths_inside_batch_calls() {
    let root = tempfile::tempdir().unwrap();
    let mut args = json!({"calls": [{
        "tool": "read", "args": {"path": "src/main.rs"}
    }]});
    normalize_tool_args("batch", &mut args, root.path()).unwrap();
    let path = args["calls"][0]["args"]["path"].as_str().unwrap();
    assert!(path.starts_with(root.path().to_str().unwrap()));
}

#[test]
fn rejects_batch_paths_outside_worktree() {
    let root = tempfile::tempdir().unwrap();
    let mut args = json!({"calls": [{
        "name": "read", "arguments": {"path": "/etc/passwd"}
    }]});
    let error = normalize_tool_args("batch", &mut args, root.path()).unwrap_err();
    assert!(error.to_string().contains("batch call 0 (read)"));
}
