//! Regression coverage for a provider that stalls after partial output.

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

use super::restart::{RestartPolicy, run};
use super::stall_test_provider::StallThenCompleteProvider;
use crate::provider::{CompletionRequest, ContentPart, Provider};

#[tokio::test(start_paused = true)]
async fn mid_stream_stall_discards_partial_and_restarts() {
    let provider: Arc<dyn Provider> = Arc::new(StallThenCompleteProvider {
        calls: AtomicUsize::new(0),
    });
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
    let response = run(&provider, request, "test-session", None, &policy)
        .await
        .unwrap();

    assert!(matches!(
        &response.message.content[0],
        ContentPart::Text { text } if text == "complete answer"
    ));
}
