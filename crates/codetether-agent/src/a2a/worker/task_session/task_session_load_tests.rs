#[tokio::test]
async fn verified_workspace_preservation_requires_the_author_session() {
    let missing = format!("missing_{}", uuid::Uuid::new_v4().simple());
    let context = format!("fallback_{}", uuid::Uuid::new_v4().simple());

    let error = super::load_ids(Some(&missing), Some(&context), true)
        .await
        .expect_err("missing verified session must not fall back");

    assert!(error.to_string().contains("Verified author session"));
    assert!(crate::session::Session::load(&context).await.is_err());
}
