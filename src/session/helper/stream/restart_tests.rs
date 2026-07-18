//! Integration test for the SRP restart engine ([`super::restart::run`]).
//!
//! Verifies that a premature end (byte stream closes before `Done`) discards a
//! committed partial and re-requests a fresh stream.

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

use super::restart::{RestartPolicy, run};
use super::restart_test_provider::FlakyThenCompleteProvider;
use crate::provider::{CompletionRequest, ContentPart, Provider};

fn empty_request() -> CompletionRequest {
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

fn zero_backoff_policy() -> RestartPolicy {
    RestartPolicy {
        max_restarts: 3,
        base_backoff: Duration::ZERO,
        multiplier: 1,
    }
}

#[tokio::test]
async fn premature_end_discards_partial_and_restarts() {
    let provider: Arc<dyn Provider> = Arc::new(FlakyThenCompleteProvider {
        calls: AtomicUsize::new(0),
    });
    let resp = run(
        &provider,
        empty_request(),
        "test-session",
        None,
        &zero_backoff_policy(),
    )
    .await
    .unwrap();
    // The truncated first pass was discarded; the complete second pass wins.
    assert!(matches!(
        &resp.message.content[0],
        ContentPart::Text { text } if text == "complete answer"
    ));
}
