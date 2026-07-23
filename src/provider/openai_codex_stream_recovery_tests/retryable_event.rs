#[tokio::test(start_paused = true)]
async fn retryable_request_failure_is_retried_before_streaming() {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    let recovered = stream_recovery::with_http_retry(move || {
        let call = observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        async move {
            if call == 0 {
                anyhow::bail!("OpenAI API error (503 Service Unavailable): overloaded");
            }
            let chunks = vec![StreamChunk::Text("recovered".into()), StreamChunk::Done { usage: None }];
            Ok(Box::pin(stream::iter(chunks)) as BoxStream<'static, StreamChunk>)
        }
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 2);
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "recovered"));
    assert!(matches!(&chunks[1], StreamChunk::Done { .. }));
}

#[tokio::test]
async fn request_retry_exhaustion_returns_to_stream_recovery() {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    let recovered = stream_recovery::with_http_retry(move || {
        observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        async { anyhow::bail!("OpenAI API error (503 Service Unavailable): overloaded") }
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 5);
    assert!(matches!(&chunks[0], StreamChunk::Error(message) if message.starts_with("codex-retryable:")));
}

#[tokio::test(start_paused = true)]
async fn overload_code_remains_retryable_after_request_retry_exhaustion() {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    let recovered = stream_recovery::with_http_retry(move || {
        observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        async {
            anyhow::bail!(
                "OpenAI API error (503 Service Unavailable): server_is_overloaded"
            )
        }
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 5);
    assert!(matches!(&chunks[0], StreamChunk::Error(message) if message.starts_with("codex-retryable:")));
}