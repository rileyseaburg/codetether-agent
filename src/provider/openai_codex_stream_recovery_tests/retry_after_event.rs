#[tokio::test(start_paused = true)]
async fn private_transport_honors_provider_retry_delay() {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    let started = tokio::time::Instant::now();
    let recovered = stream_recovery::with_http_retry(None, move || {
        let call = observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        async move {
            let chunks = if call == 0 {
                vec![StreamChunk::Error(
                    "Rate limit reached. Try again in 35 seconds.".into(),
                )]
            } else {
                vec![StreamChunk::Done { usage: None }]
            };
            Ok(Box::pin(stream::iter(chunks)) as BoxStream<'static, StreamChunk>)
        }
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 2);
    assert!(started.elapsed() >= std::time::Duration::from_secs(35));
    assert!(matches!(&chunks[0], StreamChunk::Done { .. }));
}
