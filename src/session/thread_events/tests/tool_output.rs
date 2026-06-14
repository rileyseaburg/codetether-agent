use crate::session::SessionEvent;

use super::mapper;

#[test]
fn tool_output_chunk_reuses_open_tool_id() {
    let mut mapper = mapper();
    let start = mapper.map_session_event(&SessionEvent::ToolCallStart {
        tool_call_id: "call-1".into(),
        name: "tetherscript_plugin".into(),
        arguments: "{}".into(),
    });
    let chunk = mapper.map_session_event(&SessionEvent::ToolOutputChunk {
        tool_call_id: "call-1".into(),
        name: "tetherscript_plugin".into(),
        stream: "stdout".into(),
        chunk: "line one\n".into(),
    });
    assert_eq!(chunk[0].kind, "tool.output_chunk");
    assert_eq!(start[0].payload["item_id"], chunk[0].payload["item_id"]);
    assert_eq!(chunk[0].payload["stream"], "stdout");
    assert_eq!(chunk[0].payload["chunk"], "line one\n");
}
