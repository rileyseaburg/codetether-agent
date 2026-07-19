fn assert_tool_completion(chunks: &[StreamChunk]) {
    assert_eq!(chunks.len(), 5);
    let expected = [("call_1", "{\"command\":"), ("call_1", "\"ls\"}")];
    for (chunk, (expected_id, expected_delta)) in chunks.iter().zip(expected).take(2) {
        match chunk {
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => {
                assert_eq!(id, expected_id);
                assert_eq!(arguments_delta, expected_delta);
            }
            other => panic!("expected tool delta, got {other:?}"),
        }
    }
    match &chunks[2] {
        StreamChunk::ToolCallEnd { id } => assert_eq!(id, "call_1"),
        other => panic!("expected tool end, got {other:?}"),
    }
    assert_tool_checkpoint(&chunks[3], "call_1", "bash", "{\"command\":\"ls\"}");
    match &chunks[4] {
        StreamChunk::Done { usage } => {
            let usage = usage.as_ref().expect("expected usage");
            assert_eq!(
                (
                    usage.prompt_tokens,
                    usage.completion_tokens,
                    usage.total_tokens
                ),
                (11, 7, 18)
            );
        }
        other => panic!("expected done, got {other:?}"),
    }
}
