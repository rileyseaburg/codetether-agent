fn assert_tool_start(chunks: &[StreamChunk]) {
    assert_eq!(chunks.len(), 1);
    match &chunks[0] {
        StreamChunk::ToolCallStart { id, name } => {
            assert_eq!(id, "call_1");
            assert_eq!(name, "bash");
        }
        other => panic!("expected tool start, got {other:?}"),
    }
}
