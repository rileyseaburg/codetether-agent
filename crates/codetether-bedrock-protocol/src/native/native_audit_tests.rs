use super::enforce_native;
use serde_json::json;

#[test]
fn repairs_unanswered_native_tool_use() {
    let mut body = json!({"messages": [
        {"role": "assistant", "content": [
            {"type": "tool_use", "id": "call_70YI", "name": "exec_command", "input": {}}
        ]},
        {"role": "user", "content": [{"type": "text", "text": "continue"}]}
    ]});
    assert_eq!(enforce_native(&mut body), vec!["call_70YI".to_string()]);
    assert_eq!(body["messages"][1]["content"][0]["type"], "tool_result");
    assert_eq!(
        body["messages"][1]["content"][0]["tool_use_id"],
        "call_70YI"
    );
    assert_eq!(body["messages"][1]["content"][1]["text"], "continue");
}

#[test]
fn leaves_answered_native_pairs_untouched() {
    let mut body = json!({"messages": [
        {"role": "assistant", "content": [{"type": "tool_use", "id": "x", "input": {}}]},
        {"role": "user", "content": [{"type": "tool_result", "tool_use_id": "x"}]}
    ]});
    let before = body.clone();
    assert!(enforce_native(&mut body).is_empty());
    assert_eq!(body, before);
}
