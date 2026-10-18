use super::fixture::Fixture;

#[tokio::test]
async fn dirty_verification_is_rejected_and_preserved() {
    let fixture = Fixture::new().await;
    fixture.edit_agent("unexpected\n");
    let mut result = fixture.result();
    super::super::super::apply(&mut result, &fixture.manager, &fixture.worktree, false).await;
    assert!(!result.success);
    assert!(
        result
            .error
            .as_deref()
            .unwrap()
            .contains("worktree retained")
    );
    assert_eq!(
        std::fs::read_to_string(fixture.worktree.path.join("file")).unwrap(),
        "unexpected\n"
    );
    assert!(fixture.branch_exists());
}

#[tokio::test]
async fn clean_verification_is_accepted_and_cleaned_up() {
    let fixture = Fixture::new().await;
    let mut result = fixture.result();
    super::super::super::apply(&mut result, &fixture.manager, &fixture.worktree, false).await;
    assert!(result.success, "{:?}", result.error);
    assert!(result.result.contains("No source changes to merge"));
    assert!(!fixture.worktree.path.exists());
    assert!(!fixture.branch_exists());
}
