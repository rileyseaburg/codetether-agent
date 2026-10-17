use super::{StreamChunk, chunks, tool};

#[test]
fn malformed_call_is_rejected_instead_of_rendered() {
    let error = chunks("<tool_call>{not-json}</tool_call>", &[], &[]).unwrap_err();
    assert!(error.to_string().contains("incomplete or malformed"));
}

#[test]
fn caps_calls_without_overflowing_the_stream() {
    let text = (0..12)
        .map(|index| format!(r#"<tool_call>{{"name":"t{index}","arguments":{{}}}}</tool_call>"#))
        .collect::<String>();
    let tools = (0..12)
        .map(|index| tool(&format!("t{index}")))
        .collect::<Vec<_>>();
    assert_eq!(chunks(&text, &tools, &[]).unwrap().len(), 25);
}

#[test]
fn accepts_repeated_opening_tag_as_malformed_close() {
    let text = r#"<tool_call>{"name":"pwd","arguments":{}}<tool_call>"#;
    let chunks = chunks(text, &[tool("pwd")], &[]).unwrap();
    assert!(matches!(
        &chunks[0],
        StreamChunk::ToolCallStart { name, .. } if name == "pwd"
    ));
}

#[test]
fn accepts_case_and_tag_whitespace_variants() {
    let text = r#"<TOOL_CALL >{"name":"pwd","arguments":{}}</TOOL_CALL >"#;
    assert!(matches!(
        &chunks(text, &[tool("pwd")], &[]).unwrap()[0],
        StreamChunk::ToolCallStart { name, .. } if name == "pwd"
    ));
}
