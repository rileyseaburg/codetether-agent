use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use super::restart::{RestartPolicy, run};
use super::restart_test_provider::TransientFaultThenCompleteProvider;
use crate::provider::{CompletionRequest, ContentPart, Provider};
use crate::session::SessionEvent;

#[tokio::test]
async fn transient_fault_after_partial_restarts_whole_request() {
    let concrete = Arc::new(TransientFaultThenCompleteProvider {
        calls: AtomicUsize::new(0),
    });
    let provider: Arc<dyn Provider> = concrete.clone();
    let request = CompletionRequest {
        messages: Vec::new(),
        tools: Vec::new(),
        model: "mock".into(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: Vec::new(),
    };
    let policy = RestartPolicy {
        max_restarts: 1,
        base_backoff: Duration::ZERO,
        multiplier: 1,
    };
    let (tx, mut rx) = tokio::sync::mpsc::channel(8);
    let response = run(&provider, request, "test-session", Some(&tx), &policy)
        .await
        .unwrap();
    assert_eq!(concrete.calls.load(Ordering::SeqCst), 2);
    assert!(matches!(&response.message.content[0],
        ContentPart::Text { text } if text == "complete answer"));
    assert!(
        matches!(rx.recv().await, Some(SessionEvent::TextChunk(text))
        if text == "discarded partial")
    );
    assert!(matches!(
        rx.recv().await,
        Some(SessionEvent::StreamRetry(
            crate::session::StreamRetryEvent { attempt: 1, .. }
        ))
    ));
    assert!(
        matches!(rx.recv().await, Some(SessionEvent::TextChunk(text))
        if text == "complete answer")
    );
}
