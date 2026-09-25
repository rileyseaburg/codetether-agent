//! Argument-free tool calls must not be reported as truncated.

use super::parse_tool_calls;
use crate::provider::ContentPart;

fn call(name: &str, arguments: &str) -> ContentPart {
    ContentPart::ToolCall {
        id: format!("id-{name}"),
        name: name.into(),
        arguments: arguments.into(),
        thought_signature: None,
    }
}

#[test]
fn empty_arguments_parse_as_empty_object() {
    let (parsed, truncated) = parse_tool_calls(&[call("get_goal", "")]);
    assert!(truncated.is_empty(), "argument-free call flagged truncated");
    assert_eq!(parsed[0].1, "get_goal");
    assert_eq!(parsed[0].2, serde_json::json!({}));
}

#[test]
fn cut_off_json_is_still_truncated() {
    let (parsed, truncated) = parse_tool_calls(&[call("write", "{\"path\":\"a")]);
    assert!(parsed.is_empty());
    assert_eq!(
        truncated,
        vec![("id-write".to_string(), "write".to_string())]
    );
}

#[test]
fn parse_arguments_handles_blank_valid_and_cut_off() {
    use super::parse_arguments;
    assert_eq!(parse_arguments("  ").unwrap(), serde_json::json!({}));
    assert_eq!(parse_arguments("{\"a\":1}").unwrap()["a"], 1);
    assert!(parse_arguments("{\"a\":").is_err());
}
