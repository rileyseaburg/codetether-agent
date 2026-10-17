//! Existing test_description_includes_codetether test.
use super::*;

#[test]
fn test_description_includes_codetether() {
    let desc = build_codetether_description("TestBot", None);
    assert!(desc.contains("CodeTether"));
    assert!(desc.contains("TestBot"));
    assert!(desc.contains("Vault"));
}
