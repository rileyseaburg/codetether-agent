use super::super::{enforce, unpaired};
use serde_json::json;

#[test]
fn enforce_is_a_noop_on_well_formed_bodies() {
    let mut body = json!({"messages": [
        {"role": "assistant", "content": [{"toolUse": {"toolUseId": "x"}}]},
        {"role": "user", "content": [{"toolResult": {"toolUseId": "x"}}]}
    ]});
    let before = body.clone();
    assert!(enforce(&mut body).is_empty());
    assert_eq!(body, before);
}

#[test]
fn enforce_repairs_a_trailing_assistant_tool_use() {
    let mut body = json!({"messages": [
        {"role": "assistant", "content": [{"toolUse": {"toolUseId": "tail"}}]}
    ]});
    assert_eq!(enforce(&mut body), vec!["tail".to_string()]);
    assert_eq!(body["messages"].as_array().unwrap().len(), 2);
    assert_eq!(body["messages"][1]["role"], "user");
}

#[test]
fn enforce_repairs_every_id_of_a_parallel_batch() {
    let mut body = json!({"messages": [
        {"role": "assistant", "content": [
            {"toolUse": {"toolUseId": "a"}},
            {"toolUse": {"toolUseId": "b"}}
        ]},
        {"role": "user", "content": [
            {"toolResult": {"toolUseId": "a", "content": [{"text": "ok"}]}}
        ]}
    ]});
    assert_eq!(enforce(&mut body), vec!["b".to_string()]);
    assert!(unpaired(body["messages"].as_array().unwrap()).is_none());
}
