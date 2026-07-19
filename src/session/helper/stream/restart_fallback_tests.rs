//! WebSocket retry-budget and HTTP fallback integration test.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::restart::{RestartPolicy, run};
use super::restart_test_provider::FallbackProvider;
use crate::provider::{CompletionRequest, ContentPart, Provider};

#[tokio::test(start_paused = true)]
async fn fallback_starts_after_five_reconnects_are_exhausted() {
    let concrete = Arc::new(FallbackProvider {
        calls: AtomicUsize::new(0),
        begins: AtomicUsize::new(0),
        fallback: AtomicBool::new(false),
        allow_fallback: true,
    });
    let provider: Arc<dyn Provider> = concrete.clone();
    let response = run(
        &provider,
        CompletionRequest {
            messages: Vec::new(),
            tools: Vec::new(),
            model: "mock".into(),
            temperature: None,
            top_p: None,
            max_tokens: None,
            stop: Vec::new(),
        },
        "test-session",
        None,
        &RestartPolicy::default(),
    )
    .await
    .unwrap();
    assert_eq!(concrete.calls.load(Ordering::SeqCst), 7);
    assert_eq!(concrete.begins.load(Ordering::SeqCst), 1);
    assert!(matches!(&response.message.content[0], ContentPart::Text { text } if text == "http"));
}
