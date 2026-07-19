//! Stream retry-budget exhaustion integration test.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::restart::{RestartPolicy, run};
use super::restart_test_provider::FallbackProvider;
use crate::provider::{CompletionRequest, Provider};

#[tokio::test(start_paused = true)]
async fn default_budget_makes_six_attempts_without_fallback() {
    let concrete = Arc::new(FallbackProvider {
        calls: AtomicUsize::new(0),
        begins: AtomicUsize::new(0),
        fallback: AtomicBool::new(false),
        allow_fallback: false,
    });
    let provider: Arc<dyn Provider> = concrete.clone();
    let error = run(
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
    .unwrap_err();
    assert_eq!(concrete.calls.load(Ordering::SeqCst), 6);
    assert!(error.to_string().contains("stream retry limit exhausted"));
}
