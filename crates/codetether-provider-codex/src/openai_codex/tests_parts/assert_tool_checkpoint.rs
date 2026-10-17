fn assert_tool_checkpoint(chunk: &StreamChunk, id: &str, name: &str, arguments: &str) {
    match chunk {
        StreamChunk::OutputItemDone {
            content:
                ContentPart::ToolCall {
                    id: actual_id,
                    name: actual_name,
                    arguments: actual_arguments,
                    ..
                },
        } => assert_eq!(
            (
                actual_id.as_str(),
                actual_name.as_str(),
                actual_arguments.as_str()
            ),
            (id, name, arguments)
        ),
        other => panic!("expected tool checkpoint, got {other:?}"),
    }
}
