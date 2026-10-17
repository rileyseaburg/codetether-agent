//! Default forwarding and identity behavior of provider implementations.

use super::*;

#[tokio::test]
async fn default_scoped_calls_and_identity_preserve_provider_contract() {
    let provider = Probe {
        calls: AtomicUsize::new(0),
    };
    assert_eq!(
        provider.resolved_model_identity("nova-pro"),
        "amazon.nova-pro-v1:0"
    );
    assert!(
        provider
            .complete_scoped(request(), "session")
            .await
            .unwrap_err()
            .to_string()
            .contains("completion reached")
    );
    assert!(
        provider
            .complete_stream_scoped(request(), "session")
            .await
            .is_ok()
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 2);
    assert!(provider.supports_structured_streaming());
    provider.begin_stream_recovery("session");
    assert!(!provider.try_stream_fallback(&request(), "session"));
    let error = provider
        .embed(EmbeddingRequest {
            model: "nova-pro".into(),
            inputs: Vec::new(),
        })
        .await
        .unwrap_err();
    assert!(error.to_string().contains("does not support embeddings"));
}
