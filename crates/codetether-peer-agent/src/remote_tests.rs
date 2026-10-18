#[tokio::test]
async fn rejects_context_ids_that_cannot_be_reused() {
    let result =
        super::message_or_missing("absent", "review", Some("../escape"), Some("parent"), false)
            .await
            .expect("validation result");

    assert!(!result.success);
    assert!(result.output.contains("context_id must match"));
}
