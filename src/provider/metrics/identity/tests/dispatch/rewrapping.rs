//! Rewrapping must retain adapter identity resolution without duplicating metadata.
use super::{MetricsProvider, Provider, mock, request, text};
use crate::provider::metrics::identity::inject;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn harness_identity_refreshes_stale_metadata_through_nested_wrappers() {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let inner = MetricsProvider::wrap(Arc::new(mock::Capture(captured.clone())));
    let wrapped = MetricsProvider::wrap(inner);
    assert_eq!(wrapped.resolved_model_identity("alias:model-b"), "model-b");
    let stale = inject(request("alias:model-b"), "old-provider", "old-model");
    assert!(wrapped.complete(stale).await.is_err());
    let calls = captured.lock().unwrap();
    assert_eq!(calls.len(), 1);
    let call = &calls[0].0;
    assert_eq!(call.model, "alias:model-b");
    assert_eq!(call.messages.len(), 3);
    let identity = text(&call.messages[0]);
    assert_eq!(identity.matches("<codetether-harness-identity>").count(), 1);
    assert!(identity.contains(r#""provider":"local_cuda""#));
    assert!(identity.contains(r#""model":"model-b""#));
    assert!(!identity.contains("old-provider"));
    assert!(!identity.contains("old-model"));
    assert!(!identity.contains("alias:"));
    assert_eq!(text(&call.messages[1]), "Preserve instructions");
    assert_eq!(text(&call.messages[2]), "Who are you?");
    assert!(calls[0].1.is_none());
}
