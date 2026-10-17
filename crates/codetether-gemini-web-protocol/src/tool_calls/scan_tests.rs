use super::{blocks, remove};

#[test]
fn protocol_text_inside_json_string_does_not_end_block() {
    let text = concat!(
        r#"<tool_call>{"name":"write","arguments":{"content":"literal }"#,
        r#"</tool_call> marker"}}</tool_call>"#
    );
    let found = blocks(text);
    assert_eq!(found.len(), 1);
    assert!(found[0].json.contains("literal }</tool_call> marker"));
}

#[test]
fn removal_preserves_narrative_spacing() {
    let text = "Before <tool_call>{\"name\":\"pwd\",\"arguments\":{}}</tool_call> after";
    let found = blocks(text);
    let ranges = found
        .iter()
        .map(|block| block.range.clone())
        .collect::<Vec<_>>();
    assert_eq!(remove(text, &ranges), "Before  after");
}
