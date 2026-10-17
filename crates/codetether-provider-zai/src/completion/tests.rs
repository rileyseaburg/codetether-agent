//! Application response contracts that differ from injected-client handling.
use crate::ContentPart;
use serde_json::json;

#[test]
fn application_filters_empty_content_but_injected_client_keeps_it() {
    let text = json!({"choices":[{"message":{"content":"","reasoning_content":""}}]}).to_string();
    let application = super::parse::parse(&text).unwrap();
    let injected = crate::response::parse(&text).unwrap();
    assert!(application.message.content.is_empty());
    assert_eq!(injected.message.content.len(), 2);
    assert!(
        matches!(&injected.message.content[0], ContentPart::Thinking { text, .. } if text.is_empty())
    );
}

#[test]
fn application_error_keeps_the_full_unstructured_body() {
    let text = "x".repeat(500);
    let message = crate::error_wire::full_message(reqwest::StatusCode::BAD_REQUEST, &text);
    assert!(message.ends_with(&text));
    let injected = crate::error_wire::message(reqwest::StatusCode::BAD_REQUEST, &text);
    assert!(!injected.ends_with(&text));
}
