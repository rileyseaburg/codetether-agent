//! Error display and conversion compatibility.
use super::BrowserError;

#[test]
fn javascript_stack_has_exactly_one_leading_newline() {
    let no_stack = BrowserError::JsException {
        message: "failed".into(),
        stack: None,
    };
    let stack = BrowserError::JsException {
        message: "failed".into(),
        stack: Some("at line 1".into()),
    };
    assert_eq!(no_stack.to_string(), "javascript exception: failed");
    assert_eq!(stack.to_string(), "javascript exception: failed\nat line 1");
}

#[test]
fn conversions_keep_the_underlying_error_message() {
    let error = anyhow::anyhow!("failure detail");
    assert_eq!(
        BrowserError::from(error).to_string(),
        "browser operation failed: failure detail"
    );
    let error = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let expected = format!("browser operation failed: {error}");
    assert_eq!(BrowserError::from(error).to_string(), expected);
}
