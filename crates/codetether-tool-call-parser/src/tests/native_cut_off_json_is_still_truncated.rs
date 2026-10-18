//! Original parsing regression case.
//! Argument-free tool calls must not be reported as truncated.

use crate::parse_tool_calls;

use super::native_support::*;
use codetether_provider_core::ContentPart;

#[test]
fn cut_off_json_is_still_truncated() {
    let (parsed, truncated) = parse_tool_calls(&[call("write", "{\"path\":\"a")]);
    assert!(parsed.is_empty());
    assert_eq!(
        truncated,
        vec![("id-write".to_string(), "write".to_string())]
    );
}
