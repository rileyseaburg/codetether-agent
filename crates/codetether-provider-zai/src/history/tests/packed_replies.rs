//! History-format regression expectation.
use super::*;
#[test]
fn packed_tool_results_keep_each_callers_cardinality() {
    let history = [Message {
        role: Role::Tool,
        content: vec![
            ContentPart::ToolResult {
                tool_call_id: "a".into(),
                content: "first".into(),
            },
            ContentPart::ToolResult {
                tool_call_id: "b".into(),
                content: "second".into(),
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
    assert_eq!(
        first,
        vec![json!({"role":"tool","tool_call_id":"a","content":"first"})]
    );
    assert_eq!(expanded.len(), 2);
    assert_eq!(expanded[1]["tool_call_id"], "b");
}
