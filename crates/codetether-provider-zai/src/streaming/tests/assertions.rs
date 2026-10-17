//! Assertions for the migrated tool-call stream scenario.
use crate::StreamChunk;
pub(super) fn tool_chunks(chunks: &[StreamChunk]) {
    assert_eq!(chunks.len(), 4);
    assert!(matches!(
        &chunks[0],
        StreamChunk::ToolCallStart { id, name }
            if id == "call_1" && name == "bash"
    ));
    assert!(matches!(
        &chunks[1],
        StreamChunk::ToolCallDelta { id, arguments_delta }
            if id == "call_1" && arguments_delta == "{\""
    ));
    assert!(matches!(
        &chunks[2],
        StreamChunk::ToolCallDelta { id, arguments_delta }
            if id == "call_1" && arguments_delta == "command\":\"pwd\"}"
    ));
    assert!(matches!(
        &chunks[3],
        StreamChunk::ToolCallEnd { id } if id == "call_1"
    ));
}
