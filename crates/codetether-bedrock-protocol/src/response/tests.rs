//! Existing UTF-8 response error regression.
use super::parse_converse_response;

#[test]
fn parse_converse_response_handles_multibyte_error_prefix_without_panicking() {
    let prefix = "a".repeat(299);
    let body = format!("{prefix}\u{2014}{{");

    let err = parse_converse_response(&body).unwrap_err();
    let message = err.to_string();

    assert!(message.contains("Failed to parse Bedrock response:"));
    assert!(!message.contains("byte index 300 is not a char boundary"));
}
