use super::super::tool_exchanges;
use super::fixtures::assistant;
use serde_json::json;

#[test]
fn synthesizes_result_when_assistant_is_last() {
    let mut messages = vec![assistant(&["call_x"])];
    tool_exchanges(&mut messages);
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[1]["role"], "user");
    assert_eq!(
        messages[1]["content"][0]["toolResult"]["toolUseId"],
        "call_x"
    );
    assert_eq!(messages[1]["content"][0]["toolResult"]["status"], "error");
}

#[test]
fn prepends_missing_result_to_existing_user_turn() {
    let mut messages = vec![
        assistant(&["call_a"]),
        json!({
            "role": "user", "content": [{"text": "continue"}]
        }),
    ];
    tool_exchanges(&mut messages);
    assert_eq!(
        messages[1]["content"][0]["toolResult"]["toolUseId"],
        "call_a"
    );
    assert_eq!(messages[1]["content"][1]["text"], "continue");
}
