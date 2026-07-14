#[tokio::test(start_paused = true)]
async fn retryable_openai_event_is_retried_inside_transport() {
    let primary = Box::pin(stream::iter(vec![StreamChunk::Error(
        "An error occurred while processing your request. You can retry your request. Request ID abc"
            .into(),
    )]));
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    let recovered = stream_recovery::with_http_retry(primary, move || {
        let call = observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        async move {
            let chunks = if call == 0 {
                vec![StreamChunk::Error("service unavailable".into())]
            } else {
                vec![
                    StreamChunk::Text("recovered".into()),
                    StreamChunk::Done { usage: None },
                ]
            };
            Ok(Box::pin(stream::iter(chunks)) as BoxStream<'static, StreamChunk>)
        }
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 2);
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "recovered"));
    assert!(matches!(&chunks[1], StreamChunk::Done { .. }));
}
