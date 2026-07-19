#[tokio::test(start_paused = true)]
async fn request_layer_does_not_retry_rate_limits() {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    let recovered = stream_recovery::with_http_retry(move || {
        observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        async move {
            anyhow::bail!("OpenAI API error (429 Too Many Requests): rate limited")
        }
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(matches!(&chunks[0], StreamChunk::Error(message) if message.starts_with("codex-permanent:") && message.contains("429")));
}
