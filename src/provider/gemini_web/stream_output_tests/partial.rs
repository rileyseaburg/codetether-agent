use super::{StreamChunk, chunks, tool};

#[test]
fn recovers_complete_json_when_closing_tag_is_missing() {
    let text = r#"<tool_call>{"arguments":{"cmd":"pwd"},"name":"exec_command"}"#;
    let chunks = chunks(text, &[tool("exec_command")], &[]).unwrap();
    assert!(matches!(
        &chunks[0],
        StreamChunk::ToolCallStart { name, .. } if name == "exec_command"
    ));
}

#[test]
fn recovers_complete_fenced_json_at_end_of_response() {
    let text = "<tool_call>```json\n{\"name\":\"pwd\",\"arguments\":{}}\n```";
    let chunks = chunks(text, &[tool("pwd")], &[]).unwrap();
    assert!(matches!(&chunks[0], StreamChunk::ToolCallStart { .. }));
}

#[test]
fn strips_fence_wrapping_entire_tool_block() {
    let text = "```xml\n<tool_call>{\"name\":\"pwd\",\"arguments\":{}}</tool_call>\n```";
    let chunks = chunks(text, &[tool("pwd")], &[]).unwrap();
    assert!(
        !chunks
            .iter()
            .any(|chunk| matches!(chunk, StreamChunk::Text(_)))
    );
}

#[test]
fn rejects_json_cut_off_mid_tool_call() {
    let text = r#"<tool_call>{"name":"exec_command","arguments":{"cmd":"pwd"#;
    let error = chunks(text, &[tool("exec_command")], &[]).unwrap_err();
    assert!(error.to_string().contains("incomplete or malformed"));
}

#[test]
fn rejects_whole_batch_when_later_call_is_cut_off() {
    let text = concat!(
        r#"<tool_call>{"name":"pwd","arguments":{}}</tool_call>"#,
        r#"<tool_call>{"name":"read","arguments":{"path":"src"#
    );
    let error = chunks(text, &[tool("pwd"), tool("read")], &[]).unwrap_err();
    assert!(error.to_string().contains("incomplete or malformed"));
}
