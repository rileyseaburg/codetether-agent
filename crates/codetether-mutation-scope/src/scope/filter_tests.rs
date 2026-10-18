//! Coarse command effects never become workspace ownership.

use super::{resolve, scoped_only};
use serde_json::json;

#[test]
fn default_directory_commands_do_not_acquire_workspace_leases() {
    let root = tempfile::tempdir().unwrap();
    for (tool, input) in [
        ("exec_command", json!({"cmd": "npm test"})),
        ("bash", json!({"command": "git status && git diff"})),
        ("write_stdin", json!({"chars": "continue\n"})),
        ("tetherscript_plugin", json!({"path": "task.tether"})),
        ("git", json!({"op": "commit"})),
    ] {
        let paths = super::super::paths::mutation_paths(tool, &input).unwrap();
        assert!(scoped_only(resolve(root.path(), paths)).is_none(), "{tool}");
    }
}

#[test]
fn a_mixed_batch_retains_specific_file_claims() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join(".git")).unwrap();
    let scope = resolve(root.path(), vec!["".into(), "src/replay.rs".into()]);
    let scope = scoped_only(scope).unwrap();
    assert_eq!(scope.paths, vec![std::path::PathBuf::from("src/replay.rs")]);
}

#[test]
fn explicit_file_claims_remain_coordinated() {
    let root = tempfile::tempdir().unwrap();
    let scope = scoped_only(resolve(root.path(), vec!["src/replay.rs".into()])).unwrap();
    assert_eq!(scope.paths.len(), 1);
}
