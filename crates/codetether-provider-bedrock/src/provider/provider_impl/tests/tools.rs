//! Preserve independently indexed tool calls and fragmented JSON arguments.
use super::{
    frame::{tool_delta, tool_start, tool_stop},
    server,
};
use codetether_provider_core::StreamChunk;

#[tokio::test]
async fn fable_converse_stream_keeps_tool_ids_and_argument_fragments() {
    let first = tool_start(1, "a");
    let tail = [
        tool_start(2, "b"),
        tool_delta(1, "{\"n\":"),
        tool_delta(2, "{\"n\":2}"),
        tool_delta(1, "1}"),
        tool_stop(2),
        tool_stop(1),
    ]
    .concat();
    let mut fixture = server::open(first, tail).await;
    assert!(
        matches!(server::next(&mut fixture.stream).await, StreamChunk::ToolCallStart { id, name } if id == "a" && name == "lookup")
    );
    assert!(!fixture.task.is_finished());
    fixture.release.send(()).unwrap();
    assert!(
        matches!(server::next(&mut fixture.stream).await, StreamChunk::ToolCallStart { id, name } if id == "b" && name == "lookup")
    );
    for (expected_id, expected_delta) in [("a", "{\"n\":"), ("b", "{\"n\":2}"), ("a", "1}")] {
        let StreamChunk::ToolCallDelta {
            id,
            arguments_delta,
        } = server::next(&mut fixture.stream).await
        else {
            panic!("expected argument fragment");
        };
        assert_eq!(
            (id.as_str(), arguments_delta.as_str()),
            (expected_id, expected_delta)
        );
    }
    for expected in ["b", "a"] {
        assert!(
            matches!(server::next(&mut fixture.stream).await, StreamChunk::ToolCallEnd { id } if id == expected)
        );
    }
    assert!(matches!(
        server::next(&mut fixture.stream).await,
        StreamChunk::Done { .. }
    ));
    fixture.task.join().unwrap();
}
