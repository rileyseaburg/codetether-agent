use super::status_text;

#[test]
fn justification_block_reads_as_held_not_failed() {
    let output = r#"{"error":{"code":"TOOL_JUSTIFICATION_REQUIRED","tool":"bash"}}"#;
    let text = status_text("bash", output).expect("hint");
    assert!(text.contains("needs a justification"));
    assert!(text.contains("Nothing ran"));
}

#[test]
fn denial_says_nothing_ran() {
    let output = r#"{"error":{"code":"TOOL_APPROVAL_DENIED"}}"#;
    assert!(status_text("bash", output).unwrap().contains("denied"));
}

#[test]
fn unknown_codes_and_plain_text_fall_through() {
    assert!(status_text("bash", r#"{"error":{"code":"OTHER"}}"#).is_none());
    assert!(status_text("bash", "exit 1").is_none());
}
