//! Git-marker fixtures exercise checkout identity without allocating worktrees.

use std::path::PathBuf;

#[test]
fn managed_checkout_uses_its_git_file_not_the_parent_git_directory() {
    let directory = tempfile::tempdir().unwrap();
    let parent = directory.path();
    std::fs::create_dir(parent.join(".git")).unwrap();
    let checkout = parent.join(".codetether-worktrees/child");
    std::fs::create_dir_all(checkout.join("src")).unwrap();
    std::fs::write(
        checkout.join(".git"),
        "gitdir: ../../.git/worktrees/child\n",
    )
    .unwrap();
    let parent_scope = super::resolve(parent, vec![PathBuf::new()]);
    assert_eq!(parent_scope.workspace, parent);
    for (cwd, target, expected) in [
        (checkout.clone(), PathBuf::new(), PathBuf::new()),
        (checkout.join("src"), "lib.rs".into(), "src/lib.rs".into()),
        (
            parent.into(),
            checkout.join("src/lib.rs"),
            "src/lib.rs".into(),
        ),
    ] {
        let scope = super::resolve(&cwd, vec![target]);
        assert_eq!(scope.workspace, checkout);
        assert_ne!(scope.workspace, parent_scope.workspace);
        assert_eq!(scope.paths, vec![expected]);
    }
}
