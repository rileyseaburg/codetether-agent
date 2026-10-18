//! Subprocess mutation-directory classification tests.

use serde_json::json;
use std::path::PathBuf;

#[test]
fn command_tools_claim_their_explicit_directory() {
    let exec = json!({ "cmd": "cargo fmt", "workdir": "/repo" });
    let bash = json!({ "command": "cargo fmt", "cwd": "/repo" });
    let git = json!({ "op": "commit", "cwd": "/repo" });
    for (tool, input) in [("exec_command", exec), ("bash", bash), ("git", git)] {
        let paths = super::paths::mutation_paths(tool, &input).unwrap();
        assert_eq!(paths, vec![PathBuf::from("/repo")]);
    }
}

#[test]
fn non_filesystem_tools_do_not_take_workspace_leases() {
    assert!(super::paths::mutation_paths("todo_write", &json!({})).is_none());
    let voice = json!({ "action": "speak" });
    assert!(super::paths::mutation_paths("voice", &voice).is_none());
}
