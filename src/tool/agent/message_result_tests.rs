use super::build_message_result;

#[test]
fn partial_response_with_error_stays_failed() {
    let result = build_message_result(
        "child-id".into(),
        "reviewer".into(),
        "partial finding".into(),
        String::new(),
        Vec::new(),
        Some("provider failed".into()),
    );
    assert!(!result.success);
    assert!(result.output.contains("partial finding"));
    assert!(result.output.contains("provider failed"));
}
