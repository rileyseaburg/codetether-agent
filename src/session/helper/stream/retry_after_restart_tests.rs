#[tokio::test(start_paused = true)]
async fn whole_request_restart_honors_provider_delay() {
    let concrete = std::sync::Arc::new(super::restart_test_provider::RetryAfterProvider {
        calls: std::sync::atomic::AtomicUsize::new(0),
    });
    let provider: std::sync::Arc<dyn crate::provider::Provider> = concrete.clone();
    let request = crate::provider::CompletionRequest {
        messages: Vec::new(),
        tools: Vec::new(),
        model: "mock".into(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: Vec::new(),
    };
    let started = tokio::time::Instant::now();
    super::restart::run(&provider, request, "test-session", None, &RestartPolicy::default())
        .await
        .unwrap();
    assert_eq!(
        concrete.calls.load(std::sync::atomic::Ordering::SeqCst),
        2
    );
    assert!(started.elapsed() >= std::time::Duration::from_secs(35));
}
