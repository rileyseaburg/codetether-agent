use super::super::tool_exchanges;
use serde_json::json;

#[test]
fn moves_matching_result_ahead_of_continuation_text() {
    let mut messages = vec![
        json!({"role": "assistant", "content": [
            {"toolUse": {"toolUseId": "call_ok", "name": "t", "input": {}}}
        ]}),
        json!({"role": "user", "content": [
            {"text": "Session"},
            {"toolResult": {"toolUseId": "call_ok", "content": [{"text": "ok"}]}}
        ]}),
    ];
    tool_exchanges(&mut messages);
    assert_eq!(
        messages[1]["content"][0]["toolResult"]["toolUseId"],
        "call_ok"
    );
    assert_eq!(messages[1]["content"][1]["text"], "Session");
}
