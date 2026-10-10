use super::{base_dir, validate};

#[test]
fn worktree_location_is_workspace_relative() {
    let workspace = std::path::Path::new("/workspace/project");
    assert_eq!(base_dir(workspace), workspace.join(".codetether-worktrees"));
}

#[test]
fn rejects_arbitrary_worktree_location() {
    let error = validate(
        std::path::Path::new("/workspace/project"),
        std::path::Path::new("/tmp/random-worktree"),
    )
    .expect_err("arbitrary path must fail");
    assert!(error.to_string().contains("worktree storage must be"));
}

#[cfg(unix)]
#[test]
fn rejects_symlinked_worktree_location() {
    let workspace = tempfile::tempdir().expect("workspace");
    let outside = tempfile::tempdir().expect("outside");
    std::os::unix::fs::symlink(outside.path(), base_dir(workspace.path())).expect("symlink");
    let error = validate(workspace.path(), &base_dir(workspace.path())).expect_err("symlink");
    assert!(error.to_string().contains("cannot be a symlink"));
}
