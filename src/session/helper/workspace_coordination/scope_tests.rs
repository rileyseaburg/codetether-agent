//! Repository and dangerous-root mutation-scope tests.

use std::path::PathBuf;

#[test]
fn exact_target_is_rebased_to_its_git_repository() {
    let parent = tempfile::tempdir().unwrap();
    let repo = parent.path().join("repo");
    std::fs::create_dir_all(repo.join(".git")).unwrap();
    let target = repo.join("src/lib.rs");
    let scope = super::scope::resolve(parent.path(), vec![target]);
    assert_eq!(scope.workspace, repo);
    assert_eq!(scope.paths, vec![PathBuf::from("src/lib.rs")]);
}

#[test]
fn filesystem_root_claim_requires_an_explicit_project() {
    let scope = super::scope::MutationScope {
        workspace: PathBuf::from("/"),
        paths: vec![PathBuf::new()],
    };
    assert!(super::scope::too_broad(&scope));
}

#[test]
fn explicit_non_git_directory_becomes_its_own_workspace() {
    let directory = tempfile::tempdir().unwrap();
    let scope = super::scope::resolve(directory.path(), vec![directory.path().into()]);
    assert_eq!(scope.workspace, directory.path());
    assert_eq!(scope.paths, vec![PathBuf::new()]);
}

#[test]
fn broad_scope_error_requires_an_exact_workdir() {
    let (output, success, metadata) =
        super::scope_error::result("exec_command", &PathBuf::from("/"));
    assert!(!success);
    assert!(output.contains("Set workdir/cwd to the exact repository"));
    assert_eq!(metadata.unwrap()["error_code"], "WORKTREE_SCOPE_REQUIRED");
}
