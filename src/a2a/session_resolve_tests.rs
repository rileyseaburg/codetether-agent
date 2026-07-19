use super::usable_as_session_id;

#[test]
fn accepts_uuid_like_ids() {
    assert!(usable_as_session_id("a1b2c3d4-0000-1111-2222-333344445555"));
    assert!(usable_as_session_id("ctx_42-abc"));
}

#[test]
fn rejects_unsafe_ids() {
    assert!(!usable_as_session_id(""));
    assert!(!usable_as_session_id("../escape"));
    assert!(!usable_as_session_id("has space"));
    assert!(!usable_as_session_id(&"x".repeat(129)));
}

#[tokio::test]
async fn rejects_unsafe_context_instead_of_losing_continuity() {
    let error = super::resolve_session(Some("../escape")).await.unwrap_err();
    assert!(error.to_string().contains("context_id must match"));
}

#[tokio::test]
async fn reuses_persisted_context_across_turns() {
    let id = format!("a2a_context_test_{}", uuid::Uuid::new_v4().simple());
    let mut first = super::resolve_session(Some(&id))
        .await
        .expect("new context");
    first.title = Some("remembered review".to_string());
    first.save().await.expect("save context");

    let second = super::resolve_session(Some(&id))
        .await
        .expect("resume context");
    assert_eq!(second.id, id);
    assert_eq!(second.title.as_deref(), Some("remembered review"));

    crate::session::Session::delete(&id)
        .await
        .expect("clean up context");
}
