use super::normalize_tool_args;
use serde_json::json;

#[test]
fn bash_and_git_cwd_are_authoritative() {
    let root = tempfile::tempdir().unwrap();
    for tool in ["bash", "git"] {
        let mut omitted = json!({"command": "pwd", "op": "status"});
        normalize_tool_args(tool, &mut omitted, root.path()).unwrap();
        assert_eq!(omitted["cwd"], root.path().display().to_string());

        let mut supplied = json!({"cwd": "/tmp", "command": "pwd", "op": "status"});
        normalize_tool_args(tool, &mut supplied, root.path()).unwrap();
        assert_eq!(supplied["cwd"], root.path().display().to_string());
    }
}

#[test]
fn exact_git_and_edit_path_fields_are_normalized() {
    let root = tempfile::tempdir().unwrap();
    let mut git = json!({"op":"commit", "path":"src", "paths":["a.rs", "b.rs"]});
    normalize_tool_args("git", &mut git, root.path()).unwrap();
    assert!(
        git["path"]
            .as_str()
            .unwrap()
            .starts_with(root.path().to_str().unwrap())
    );
    assert!(
        git["paths"][0]
            .as_str()
            .unwrap()
            .starts_with(root.path().to_str().unwrap())
    );

    let mut edit = json!({"path":"src/lib.rs", "old_string":"a", "new_string":"b"});
    normalize_tool_args("edit", &mut edit, root.path()).unwrap();
    assert!(
        edit["path"]
            .as_str()
            .unwrap()
            .starts_with(root.path().to_str().unwrap())
    );
}
