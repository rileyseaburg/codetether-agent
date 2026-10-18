//! Original parsing regression case.
//! Argument-free tool calls must not be reported as truncated.

use crate::parse_tool_calls;

use super::native_support::*;
use codetether_provider_core::ContentPart;

#[test]
fn parse_arguments_handles_blank_valid_and_cut_off() {
    use crate::parse_arguments;
    assert_eq!(parse_arguments("  ").unwrap(), serde_json::json!({}));
    assert_eq!(parse_arguments("{\"a\":1}").unwrap()["a"], 1);
    assert!(parse_arguments("{\"a\":").is_err());
}
