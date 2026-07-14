use super::{working_directory, workspace};
use serde_json::json;

#[test]
fn workspace_inherits_the_parent_directory() {
    let params = json!({"__ct_parent_workspace": "/tmp/parent-worktree"});
    assert_eq!(
        workspace(&params),
        std::path::Path::new("/tmp/parent-worktree")
    );
}

#[test]
fn mutating_task_fails_when_isolation_is_unavailable() {
    let parent = std::path::Path::new("/tmp/shared");
    assert!(working_directory(None, false, parent).is_err());
    assert_eq!(working_directory(None, true, parent).unwrap(), parent);
}
