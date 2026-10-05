//! Regressions for requests resumed with an interrupted parallel tool batch.

use super::super::OpenAiCodexProvider;
use crate::provider::Message;
use serde_json::Value;

#[test]
fn missing_results_are_explicit_unknown_failures_without_changing_history() {
    let messages: Vec<Message> = serde_json::from_value(serde_json::json!([
        {"role":"assistant", "content":[
            {"type":"tool_call", "id":"missing", "name":"exec_command", "arguments":"{}"},
            {"type":"tool_call", "id":"finished", "name":"exec_command", "arguments":"{}"},
            {"type":"tool_call", "id":"also-missing", "name":"exec_command", "arguments":"{}"}
        ]},
        {"role":"tool", "content":[
            {"type":"tool_result", "tool_call_id":"finished", "content":"actual result"}
        ]}
    ]))
    .unwrap();

    let before = serde_json::to_value(&messages).unwrap();
    let wire = OpenAiCodexProvider::convert_messages_to_responses_input(&messages);
    let outputs: Vec<_> = wire
        .iter()
        .filter(|item| item["type"] == "function_call_output")
        .collect();
    assert_eq!(outputs.len(), 3);
    for id in ["missing", "also-missing"] {
        let output = outputs.iter().find(|item| item["call_id"] == id).unwrap();
        let failure: Value = serde_json::from_str(output["output"].as_str().unwrap()).unwrap();
        assert_eq!(failure["success"], false);
        assert_eq!(failure["error"], "tool_result_unavailable");
    }
    assert_eq!(
        outputs
            .iter()
            .find(|item| item["call_id"] == "finished")
            .unwrap()["output"],
        "actual result"
    );
    assert_eq!(serde_json::to_value(&messages).unwrap(), before);
}
