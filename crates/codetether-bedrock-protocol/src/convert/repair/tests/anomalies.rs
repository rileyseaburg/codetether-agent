use super::super::tool_exchanges;
use serde_json::json;

#[test]
fn removes_delayed_and_unknown_results() {
    let mut messages = vec![
        json!({"role": "assistant", "content": [
            {"toolUse": {"toolUseId": "call_x", "name": "t", "input": {}}}
        ]}),
        json!({"role": "user", "content": [{"text": "resume"}]}),
        json!({"role": "assistant", "content": [{"text": "later"}]}),
        json!({"role": "user", "content": [
            {"toolResult": {"toolUseId": "call_x", "content": [{"text": "late"}]}},
            {"toolResult": {"toolUseId": "unknown", "content": [{"text": "bad"}]}},
            {"text": "keep"}
        ]}),
    ];
    tool_exchanges(&mut messages);
    assert_eq!(
        messages[1]["content"][0]["toolResult"]["toolUseId"],
        "call_x"
    );
    assert_eq!(messages[1]["content"][0]["toolResult"]["status"], "error");
    assert_eq!(messages[3]["content"], json!([{"text": "keep"}]));
}

#[test]
fn removes_duplicate_matching_results() {
    let mut messages = paired_messages();
    let expected = messages[1]["content"][0].clone();
    let duplicate = messages[1]["content"][0].clone();
    messages[1]["content"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    tool_exchanges(&mut messages);
    assert_eq!(messages[1]["content"].as_array().unwrap().len(), 1);
    assert_eq!(messages[1]["content"][0], expected);
}

fn paired_messages() -> Vec<serde_json::Value> {
    vec![
        json!({"role": "assistant", "content": [{"toolUse": {"toolUseId": "x"}}]}),
        json!({"role": "user", "content": [{"toolResult": {"toolUseId": "x"}}]}),
    ]
}
