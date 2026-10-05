//! Stream-level regressions for cache-aware cumulative usage.
use super::super::sse_message_delta::parse;
use crate::provider::StreamChunk;
use futures::StreamExt;

#[tokio::test]
async fn streaming_preserves_cached_input_and_uses_cumulative_output() {
    let lines = [
        r#"data: {"type":"message_start","message":{"usage":{"input_tokens":10,"output_tokens":1,"cache_read_input_tokens":100,"cache_creation_input_tokens":20}}}"#,
        r#"data: {"type":"message_delta","usage":{"output_tokens":5}}"#,
        "",
    ];
    let chunks: Vec<_> = Box::pin(super::tests::stream_from_sse(&lines))
        .collect()
        .await;
    let usage = chunks
        .iter()
        .find_map(|chunk| match chunk {
            StreamChunk::Done { usage } => usage.as_ref(),
            _ => None,
        })
        .expect("final usage");
    assert_eq!(usage.prompt_tokens, 10);
    assert_eq!(usage.completion_tokens, 5);
    assert_eq!(usage.cache_read_tokens, Some(100));
    assert_eq!(usage.cache_write_tokens, Some(20));
    assert_eq!(usage.total_tokens, 135);
}

#[test]
fn absent_delta_usage_keeps_start_counters() {
    let mut usage = None;
    let start = serde_json::json!({"type":"message_start","message":{"usage":{
        "input_tokens":7,"cache_read_input_tokens":40
    }}});
    assert!(parse(&start, &mut usage).is_none());
    let delta = serde_json::json!({"type":"message_delta"});
    let Some(StreamChunk::Done { usage: Some(usage) }) = parse(&delta, &mut usage) else {
        panic!("expected retained usage")
    };
    assert_eq!(usage.prompt_tokens, 7);
    assert_eq!(usage.total_tokens, 47);
}

#[test]
fn missing_usage_remains_unknown() {
    let delta = serde_json::json!({"type":"message_delta"});
    assert!(matches!(
        parse(&delta, &mut None),
        Some(StreamChunk::Done { usage: None })
    ));
}
