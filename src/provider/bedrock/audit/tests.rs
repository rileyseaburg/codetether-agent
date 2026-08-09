use super::{enforce, unpaired};
use serde_json::json;

#[test]
fn detects_unpaired_tool_use_after_cache_point_insertion() {
    let messages = vec![
        json!({"role": "assistant", "content": [
            {"toolUse": {"toolUseId": "call_70YI", "name": "exec_command", "input": {}}}
        ]}),
        json!({"role": "user", "content": [{"cachePoint": {"type": "default"}}]}),
    ];
    let (index, missing) = unpaired(&messages).expect("violation detected");
    assert_eq!(index, 0);
    assert_eq!(missing, vec!["call_70YI".to_string()]);
}

#[test]
fn enforce_repairs_body_and_reports_synthesized_ids() {
    let mut body = json!({"messages": [
        {"role": "assistant", "content": [
            {"toolUse": {"toolUseId": "call_70YI", "name": "exec_command", "input": {}}}
        ]},
        {"role": "user", "content": [{"text": "continue"}]}
    ]});
    let synthesized = enforce(&mut body);
    assert_eq!(synthesized, vec!["call_70YI".to_string()]);
    assert_eq!(
        body["messages"][1]["content"][0]["toolResult"]["toolUseId"],
        "call_70YI"
    );
    assert_eq!(body["messages"][1]["content"][1]["text"], "continue");
    assert!(unpaired(body["messages"].as_array().unwrap()).is_none());
}

#[cfg(test)]
#[path = "tests_shapes.rs"]
mod shapes;
