//! Local title inputs retain the lifecycle's empty/bootstrap filtering rules.
use super::*;

#[tokio::test]
async fn absent_or_blank_user_text_does_not_call_providers() {
    for name in ["bedrock", "selected"] {
        let provider = provider(name, None);
        let mut session = Session::new("  \n ");
        generate_with(&mut session, &provider, "selected-model")
            .await
            .unwrap();
        session.messages.clear();
        generate_with(&mut session, &provider, "selected-model")
            .await
            .unwrap();
        assert!(session.title.is_none());
        assert_eq!(session.touches, 0);
        assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn bedrock_skips_bootstrap_context_and_uses_the_first_eligible_message() {
    let provider = provider("bedrock", None);
    let mut session = Session::new("# AGENTS.md instructions for /workspace");
    session
        .messages
        .extend(Session::new("Review storage").messages);
    generate_with(&mut session, &provider, "selected-model")
        .await
        .unwrap();
    assert_eq!(session.title.as_deref(), Some("Review storage"));
    assert_eq!(session.touches, 1);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
}
