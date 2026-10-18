//! Paste buffer regression retained from native input handling.
use crate::*;

#[test]
fn should_summarize_thresholds() {
    // 2 lines, 22 bytes — stay inline.
    assert!(!should_summarize("first line\nsecond line"));
    // 3 lines, 28 bytes — stay inline.
    assert!(!should_summarize("line one\nline two\nline three"));
    // 5 lines — summarise.
    assert!(should_summarize("a\nb\nc\nd\ne"));
    // 1 line, 400+ bytes — summarise.
    let big = "x".repeat(400);
    assert!(should_summarize(&big));
    // 1 line, 399 bytes — stay inline.
    let almost = "x".repeat(399);
    assert!(!should_summarize(&almost));
}
