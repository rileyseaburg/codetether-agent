//! Preserve current event selection while moving stream ownership.
use crate::StreamChunk;

#[test]
fn multiple_events_keep_the_existing_first_only_selection() {
    let mut chunks = Vec::new();
    super::line::push(
        "data: {\"choices\":[{\"delta\":{\"content\":\"first\"},\"finish_reason\":null}]}",
        &mut chunks,
    );
    super::line::push(
        "data: {\"choices\":[{\"delta\":{\"content\":\"second\"},\"finish_reason\":null}]}",
        &mut chunks,
    );
    assert_eq!(chunks.len(), 2);
    assert!(matches!(super::select::first(chunks), StreamChunk::Text(text) if text == "first"));
}

#[test]
fn done_line_retains_its_terminal_event() {
    let mut chunks = Vec::new();
    super::line::push("data: [DONE]", &mut chunks);
    assert!(matches!(
        super::select::first(chunks),
        StreamChunk::Done { usage: None }
    ));
}
