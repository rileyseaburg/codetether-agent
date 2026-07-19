use super::MetricsProvider;
use crate::provider::{CompletionRequest, Provider};
use std::sync::Arc;

#[path = "provider_impl_tests/mock.rs"]
mod mock;

fn request() -> CompletionRequest {
    CompletionRequest {
        messages: Vec::new(),
        tools: Vec::new(),
        model: "mock".into(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: Vec::new(),
    }
}

#[test]
fn delegates_stream_recovery_hooks() {
    let inner = Arc::new(mock::RecoveryProvider::default());
    let wrapped = MetricsProvider::wrap(inner.clone());
    wrapped.begin_stream_recovery("session");
    assert_eq!(inner.begins.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(wrapped.try_stream_fallback(&request(), "session"));
    assert_eq!(inner.fallbacks.load(std::sync::atomic::Ordering::SeqCst), 1);
}
