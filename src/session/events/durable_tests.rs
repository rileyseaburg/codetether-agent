use super::SessionEvent;

#[test]
fn tool_and_terminal_events_are_durable() {
    assert!(
        SessionEvent::ToolCallStart {
            tool_call_id: "call-1".into(),
            name: "bash".into(),
            arguments: "{}".into(),
        }
        .is_durable()
    );
    assert!(SessionEvent::TextComplete("done".into()).is_durable());
    assert!(SessionEvent::Done.is_durable());
}

#[test]
fn streaming_chunks_stay_ephemeral() {
    assert!(!SessionEvent::TextChunk("partial".into()).is_durable());
    assert!(!SessionEvent::Thinking.is_durable());
}
