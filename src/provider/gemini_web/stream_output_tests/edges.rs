use super::{StreamChunk, chunks};

#[test]
fn malformed_call_remains_visible_and_is_not_dispatched() {
    let chunks = chunks("<tool_call>{not-json}</tool_call>");
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text.contains("not-json")));
    assert!(
        !chunks
            .iter()
            .any(|chunk| matches!(chunk, StreamChunk::ToolCallStart { .. }))
    );
}

#[test]
fn converts_more_calls_than_the_old_channel_capacity() {
    let text = (0..12)
        .map(|index| format!(r#"<tool_call>{{"name":"t{index}","arguments":{{}}}}</tool_call>"#))
        .collect::<String>();
    assert_eq!(chunks(&text).len(), 37);
}

#[test]
fn accepts_repeated_opening_tag_as_malformed_close() {
    let text = r#"<tool_call>{"name":"pwd","arguments":{}}<tool_call>"#;
    let chunks = chunks(text);
    assert!(matches!(
        &chunks[0],
        StreamChunk::ToolCallStart { name, .. } if name == "pwd"
    ));
}

#[test]
fn accepts_case_and_tag_whitespace_variants() {
    let text = r#"<TOOL_CALL >{"name":"pwd","arguments":{}}</TOOL_CALL >"#;
    assert!(matches!(
        &chunks(text)[0],
        StreamChunk::ToolCallStart { name, .. } if name == "pwd"
    ));
}
