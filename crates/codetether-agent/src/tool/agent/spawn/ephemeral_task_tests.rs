//! Read-only ephemeral tasks share cwd; writable tasks require managed isolation.

use crate::tool::agent::spawn_request::SpawnRequest;
#[path = "../spawn_git_fixture.rs"]
mod git;
#[path = "ephemeral_task_test_support.rs"]
mod support;

#[tokio::test]
async fn read_only_ephemeral_policy_shares_even_non_git_workspace() {
    let workspace = tempfile::tempdir().unwrap();
    let params = support::params(workspace.path(), "Inspect the API");
    let request = SpawnRequest::from_params(&params).unwrap();
    let (cwd, read_only, expects_changes, handoff) = super::policy(&request).await.unwrap();
    assert_eq!(cwd, workspace.path());
    assert!(read_only);
    assert!(!expects_changes);
    assert!(handoff.is_none());
    assert!(!workspace.path().join(".codetether-worktrees").exists());
}

#[tokio::test]
async fn writable_ephemeral_policy_isolates_mutation_and_verification() {
    let repo = git::fixture();
    let primary = repo.path().canonicalize().unwrap();
    for (instruction, changes) in [("Fix the API", true), ("Run focused tests", false)] {
        let params = support::params(&primary, instruction);
        let request = SpawnRequest::from_params(&params).unwrap();
        let (cwd, read_only, expects_changes, handoff) = super::policy(&request).await.unwrap();
        assert!(!read_only);
        assert_eq!(expects_changes, changes);
        let handoff = handoff.expect("writable task requires a checkout");
        assert_eq!(cwd, handoff.workspace);
        assert_ne!(cwd, primary);
        assert_eq!(handoff.parent_workspace, primary);
        assert_eq!(
            handoff.worktree.parent(),
            Some(primary.join(".codetether-worktrees").as_path())
        );
        assert!(cwd.is_dir());
        std::fs::write(cwd.join("child-only.txt"), "isolated").unwrap();
        assert!(!primary.join("child-only.txt").exists());
    }
}
