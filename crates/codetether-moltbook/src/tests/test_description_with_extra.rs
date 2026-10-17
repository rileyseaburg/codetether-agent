//! Existing test_description_with_extra test.
use super::*;

#[test]
fn test_description_with_extra() {
    let desc = build_codetether_description("TestBot", Some("also does math"));
    assert!(desc.contains("CodeTether"));
    assert!(desc.contains("also does math"));
}
