use super::fixture::Fixture;

#[tokio::test]
async fn empty_or_reverted_mutation_is_rejected_and_retained() {
    let fixture = Fixture::new().await;
    fixture.edit_agent("changed\n");
    fixture.commit_agent("change");
    fixture.edit_agent("base\n");
    fixture.commit_agent("revert");
    let mut result = fixture.result();
    super::super::super::apply(&mut result, &fixture.manager, &fixture.worktree, true).await;
    assert!(!result.success);
    assert!(
        result
            .error
            .as_deref()
            .unwrap()
            .contains("without mergeable")
    );
    assert!(fixture.worktree.path.exists());
    assert!(fixture.branch_exists());
}

#[tokio::test]
async fn patch_equivalent_work_is_accepted_and_cleaned_up() {
    let fixture = Fixture::new().await;
    fixture.edit_agent("same\n");
    fixture.commit_agent("agent change");
    fixture.edit_main("same\n", "main change");
    let mut result = fixture.result();
    super::super::super::apply(&mut result, &fixture.manager, &fixture.worktree, true).await;
    assert!(result.success, "{:?}", result.error);
    assert!(result.result.contains("no changes to merge"));
    assert!(!fixture.worktree.path.exists());
    assert!(!fixture.branch_exists());
}
