use super::super::tool_exchanges;
use super::fixtures::assistant;

#[test]
fn handles_multiple_missing_ids() {
    let mut messages = vec![assistant(&["call_1", "call_2"])];
    tool_exchanges(&mut messages);
    assert_eq!(messages[1]["content"].as_array().unwrap().len(), 2);
    assert_eq!(
        messages[1]["content"][0]["toolResult"]["toolUseId"],
        "call_1"
    );
    assert_eq!(
        messages[1]["content"][1]["toolResult"]["toolUseId"],
        "call_2"
    );
}
