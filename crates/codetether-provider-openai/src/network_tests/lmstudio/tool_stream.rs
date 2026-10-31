//! Streamed tool IDs must survive later fragments that contain only an index.
use super::*;

#[tokio::test]
async fn lmstudio_interleaved_tool_arguments_keep_original_ids() {
    let deltas = [
        json!({"tool_calls":[{"index":0,"id":"read-id","function":{"name":"read","arguments":""}}]}),
        json!({"tool_calls":[{"index":1,"id":"list-id","function":{"name":"list","arguments":""}}]}),
        json!({"tool_calls":[{"index":0,"function":{"arguments":"{\"path\":"}}]}),
        json!({"tool_calls":[{"index":1,"function":{"arguments":"{\"path\":\".\"}"}}]}),
        json!({"tool_calls":[{"index":0,"function":{"arguments":"\"docs/lmstudio.md\",\"limit\":5}"}}]}),
    ];
    let payload = deltas.into_iter().map(frame).collect::<String>() + "data: [DONE]\n\n";
    let (url, server) = serve(vec![
        metadata(32768),
        reply(200, "text/event-stream", payload),
    ]);
    let chunks: Vec<_> = provider(url)
        .complete_stream(request(MODEL))
        .await
        .unwrap()
        .collect()
        .await;
    let mut starts = Vec::new();
    let mut read_args = String::new();
    let mut list_args = String::new();
    for chunk in &chunks {
        match chunk {
            StreamChunk::ToolCallStart { id, name } => starts.push((id.as_str(), name.as_str())),
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => match id.as_str() {
                "read-id" => read_args.push_str(arguments_delta),
                "list-id" => list_args.push_str(arguments_delta),
                other => panic!("spurious tool ID: {other}"),
            },
            StreamChunk::Done { .. } => {}
            other => panic!("unexpected chunk: {other:?}"),
        }
    }
    assert_eq!(starts, [("read-id", "read"), ("list-id", "list")]);
    assert_eq!(
        serde_json::from_str::<Value>(&read_args).unwrap(),
        json!({"path":"docs/lmstudio.md","limit":5})
    );
    assert_eq!(
        serde_json::from_str::<Value>(&list_args).unwrap(),
        json!({"path":"."})
    );
    assert!(matches!(chunks.last(), Some(StreamChunk::Done { .. })));
    assert_eq!(server.join().unwrap().len(), 2);
}
