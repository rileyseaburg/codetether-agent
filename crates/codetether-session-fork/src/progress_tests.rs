#[test]
fn independent_branch_does_not_inherit_parent_retry_restrictions() {
    let mut parent = codetether_session_metadata::SessionMetadata::default();
    parent.progress.observations = 20;
    parent.progress.stagnant_results = 17;
    let fork = super::metadata::branch_metadata(&parent, "fork", "build");
    assert_eq!(fork.progress.observations, 0);
    assert_eq!(fork.progress.stagnant_results, 0);
    assert!(fork.run_checkpoint_cleared);
}
