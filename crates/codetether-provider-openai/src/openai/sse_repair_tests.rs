//! Tests for tool-call/result sequence repair.

use super::sse_repair::repair_tool_results;
use serde_json::{Value, json};

fn roles(msgs: &[Value]) -> Vec<&str> {
    msgs.iter()
        .map(|m| m.get("role").and_then(Value::as_str).unwrap_or(""))
        .collect()
}

#[test]
fn dangling_tool_call_gets_synthetic_result() {
    let input = vec![
        json!({"role":"assistant","tool_calls":[{"id":"c1","type":"function","function":{"name":"read","arguments":"{}"}}]}),
        json!({"role":"user","content":"continue"}),
    ];
    let out = repair_tool_results(input);
    assert_eq!(roles(&out), vec!["assistant", "tool", "user"]);
    assert_eq!(out[1]["tool_call_id"], "c1");
}

#[test]
fn answered_tool_call_is_untouched() {
    let input = vec![
        json!({"role":"assistant","tool_calls":[{"id":"c1","type":"function","function":{"name":"read","arguments":"{}"}}]}),
        json!({"role":"tool","tool_call_id":"c1","content":"ok"}),
    ];
    let out = repair_tool_results(input.clone());
    assert_eq!(out.len(), 2);
    assert_eq!(roles(&out), vec!["assistant", "tool"]);
}

#[test]
fn plain_conversation_unchanged() {
    let input = vec![
        json!({"role":"user","content":"hi"}),
        json!({"role":"assistant","content":"hello"}),
    ];
    let out = repair_tool_results(input.clone());
    assert_eq!(out, input);
}
