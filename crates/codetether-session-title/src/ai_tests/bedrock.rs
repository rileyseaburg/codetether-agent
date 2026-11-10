//! Bedrock automatic naming avoids inference and preserves manual titles.
use super::*;

#[tokio::test]
async fn bedrock_generates_locally_without_provider_calls() {
    for model in [
        "us.anthropic.claude-fable-5-2",
        "us.anthropic.claude-opus-5-5",
        "amazon.nova-pro-v1:0",
    ] {
        let provider = provider("bedrock", None);
        let mut session = Session::new("  Review\nstorage  ");
        generate_with(&mut session, &provider, model).await.unwrap();
        assert_eq!(session.title.as_deref(), Some("Review storage"));
        assert_eq!(session.touches, 1);
        assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    }
}

#[tokio::test]
async fn existing_titles_are_not_replaced_or_touched() {
    for name in ["bedrock", "selected"] {
        for title in ["My manual title", ""] {
            let provider = provider(name, None);
            let mut session = Session::new("Review storage");
            session.title = Some(title.into());
            generate_with(&mut session, &provider, "selected-model")
                .await
                .unwrap();
            assert_eq!(session.title.as_deref(), Some(title));
            assert_eq!(session.touches, 0);
            assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
        }
    }
}
