#[test]
fn message_call_names_the_collaborator() {
    let value = serde_json::json!({
        "action": "message", "name": "voice-owner", "message": "Share the join schema"
    });
    assert_eq!(
        super::arg_preview_agent::format(&value),
        "ask @voice-owner — Share the join schema"
    );
}

#[test]
fn roster_result_is_one_clean_line() {
    let output = serde_json::json!([
        {"name": "voice-owner"}, {"name": "ios-owner"}
    ])
    .to_string();
    assert_eq!(
        super::result_preview::format("agent", &output),
        "2 collaborators ready: @voice-owner, @ios-owner"
    );
}

#[test]
fn non_agent_output_is_untouched() {
    let noisy = "stderr: Terminated: 15";
    assert_eq!(super::result_preview::format("bash", noisy), noisy);
}
