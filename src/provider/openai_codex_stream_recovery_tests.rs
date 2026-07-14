#[tokio::test]
async fn empty_primary_switches_to_recovery_stream() {
    let primary = Box::pin(stream::empty());
    let recovered = stream_recovery::with_http_retry(Some(primary), || async {
        Ok(Box::pin(stream::iter(vec![
            StreamChunk::Text("recovered".into()),
            StreamChunk::Done { usage: None },
        ])) as BoxStream<'static, StreamChunk>)
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "recovered"));
    assert!(matches!(&chunks[1], StreamChunk::Done { .. }));
}

#[tokio::test]
async fn completed_primary_does_not_open_recovery_stream() {
    let primary = Box::pin(stream::iter(vec![StreamChunk::Done { usage: None }]));
    let opened = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let observed = Arc::clone(&opened);
    let recovered = stream_recovery::with_http_retry(Some(primary), move || {
        let observed = Arc::clone(&observed);
        async move {
            observed.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(Box::pin(stream::empty()) as BoxStream<'static, StreamChunk>)
        }
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert!(matches!(&chunks[0], StreamChunk::Done { .. }));
    assert!(!opened.load(std::sync::atomic::Ordering::SeqCst));
}

include!("openai_codex_stream_recovery_tests/retryable_event.rs");
