use super::latest;
use serde_json::json;

fn frame(text: &str) -> String {
    let inner = json!([null, null, null, null, [[null, [text]]]]);
    json!([["wrb.fr", null, inner.to_string()]]).to_string()
}

#[test]
fn selects_latest_cumulative_candidate() {
    let raw = [frame("<tool"), frame("<tool_call>{}")].join("\n");
    assert_eq!(latest(&raw), "<tool_call>{}");
}

#[test]
fn prefers_shorter_correction_over_abandoned_long_draft() {
    let draft = "<tool_call>{\"name\":\"exec_command\",\"arguments\":{";
    let raw = [frame(draft), frame("I need more information.")].join("\n");
    assert_eq!(latest(&raw), "I need more information.");
}

#[test]
fn ignores_xssi_lengths_and_malformed_frames() {
    let raw = format!(")]}}'\n123\nnot-json\n{}", frame("valid"));
    assert_eq!(latest(&raw), "valid");
}
