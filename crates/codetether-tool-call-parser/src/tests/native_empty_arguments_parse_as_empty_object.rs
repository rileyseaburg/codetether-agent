//! Original parsing regression case.
//! Argument-free tool calls must not be reported as truncated.

use crate::parse_tool_calls;

use super::native_support::*;
use codetether_provider_core::ContentPart;

#[test]
fn empty_arguments_parse_as_empty_object() {
    let (parsed, truncated) = parse_tool_calls(&[call("get_goal", "")]);
    assert!(truncated.is_empty(), "argument-free call flagged truncated");
    assert_eq!(parsed[0].1, "get_goal");
    assert_eq!(parsed[0].2, serde_json::json!({}));
}
