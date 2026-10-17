#[tokio::test]
async fn successful_request_forwards_stream() {
    let recovered = stream_recovery::with_http_retry(|| async {
        Ok(Box::pin(stream::iter(vec![
            StreamChunk::Text("complete".into()),
            StreamChunk::Done { usage: None },
        ])) as BoxStream<'static, StreamChunk>)
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "complete"));
    assert!(matches!(&chunks[1], StreamChunk::Done { .. }));
}

#[tokio::test]
async fn midstream_error_is_left_for_turn_recovery() {
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = Arc::clone(&calls);
    let recovered = stream_recovery::with_http_retry(move || {
        observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        async move {
            Ok(Box::pin(stream::iter([
                StreamChunk::Text("partial".into()),
                StreamChunk::Error("connection reset".into()),
            ])) as BoxStream<'static, StreamChunk>)
        }
    });
    let chunks = recovered.collect::<Vec<_>>().await;
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(matches!(&chunks[1], StreamChunk::Error(_)));
}

include!("openai_codex_stream_recovery_tests/retryable_event.rs");
include!("openai_codex_stream_recovery_tests/retry_after_event.rs");
