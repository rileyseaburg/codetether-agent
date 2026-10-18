use super::build;

#[test]
fn preserves_remote_conversation_context() {
    let payload = build("review reply", Some("forgejo-repo-pr-42"));
    assert_eq!(
        payload.message.context_id.as_deref(),
        Some("forgejo-repo-pr-42")
    );
    assert_eq!(payload.configuration.unwrap().blocking, Some(false));
}
