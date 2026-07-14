use super::{create_agent_session, parent_prior_context_allowed};

#[tokio::test]
async fn child_session_inherits_denial_and_enables_auto_apply() {
    let session = create_agent_session("child", "edit files", "example/model", None, false)
        .await
        .expect("child session");
    assert!(session.metadata.auto_apply_edits);
    assert_eq!(
        session.metadata.inherited_prior_context_allowed,
        Some(false)
    );
}

#[tokio::test]
async fn live_parent_ceiling_wins_without_disk_lookup() {
    assert!(!parent_prior_context_allowed(Some(false), None).await);
}

#[tokio::test]
async fn missing_parent_policy_fails_closed() {
    let id = format!("missing-{}", uuid::Uuid::new_v4());
    assert!(!parent_prior_context_allowed(None, Some(&id)).await);
}
