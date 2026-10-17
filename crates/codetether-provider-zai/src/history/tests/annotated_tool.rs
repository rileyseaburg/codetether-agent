//! History-format regression expectation.
use super::*;
#[test]
fn non_result_first_part_retains_application_empty_reply() {
    let history = [Message {
        role: Role::Tool,
        content: vec![
            ContentPart::Text {
                text: "annotation".into(),
            },
            ContentPart::ToolResult {
                tool_call_id: "a".into(),
                content: "result".into(),
            },
        ],
    }];
    let first = convert(&history, false, HistoryFormat::SingleReply, str::to_owned);
    let expanded = convert(
        &history,
        false,
        HistoryFormat::ExpandedReplies,
        str::to_owned,
    );
    assert_eq!(first, vec![json!({"role":"tool","content":""})]);
    assert_eq!(expanded[0]["tool_call_id"], "a");
}
