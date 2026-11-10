//! Other providers retain AI naming/fallback; explicit Bedrock AI is unchanged.
use super::*;

#[tokio::test]
async fn other_providers_still_use_selected_model() {
    for name in ["selected", "anthropic"] {
        let provider = provider(name, Some("\"Generated title\"."));
        let mut session = Session::new("Review storage");
        generate_with(&mut session, &provider, "selected-model")
            .await
            .unwrap();
        assert_eq!(session.title.as_deref(), Some("Generated title"));
        assert_eq!(session.touches, 1);
        assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    }
}

#[tokio::test]
async fn other_providers_fall_back_on_failure_or_empty_response() {
    for reply in [None, Some("  \"\"  ")] {
        let provider = provider("selected", reply);
        let mut session = Session::new("Review storage");
        generate_with(&mut session, &provider, "selected-model")
            .await
            .unwrap();
        assert_eq!(session.title.as_deref(), Some("Review storage"));
        assert_eq!(session.touches, 1);
        assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    }
}

#[tokio::test]
async fn explicit_bedrock_title_request_still_calls_provider() {
    let provider = provider("bedrock", Some("Generated title"));
    let title = crate::title_from_model(&provider, "selected-model", "Review storage")
        .await
        .unwrap();
    assert_eq!(title.as_deref(), Some("Generated title"));
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
}
