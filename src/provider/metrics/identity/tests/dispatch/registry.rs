//! Default routing and provider aliases must report the canonical provider identity.
use super::{mock, request, text};
use crate::provider::ProviderRegistry;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn harness_identity_uses_registry_defaults_and_canonical_provider_aliases() {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let mut registry = ProviderRegistry::new();
    registry.register(Arc::new(mock::Capture(captured.clone())));
    for routing in ["model-a", "local-cuda/model-a", "localcuda/model-a"] {
        let (provider, model) = registry.resolve_model(routing).unwrap();
        assert_eq!(model, "model-a");
        assert!(provider.complete(request(&model)).await.is_err());
    }
    let calls = captured.lock().unwrap();
    assert_eq!(calls.len(), 3);
    for (call, _) in calls.iter() {
        assert!(text(&call.messages[0]).contains(r#""provider":"local_cuda""#));
        assert!(text(&call.messages[0]).contains(r#""model":"model-a""#));
    }
}
