//! Tests for Z.AI stream output tracking.

use super::OutputSeen;

#[test]
fn default_is_empty() {
    assert!(OutputSeen::default().is_empty());
}

#[test]
fn tool_only_turn_is_not_empty() {
    let seen = OutputSeen {
        text: false,
        tool_calls: true,
    };
    assert!(!seen.is_empty(), "a tool-call-only turn produced output");
}

#[test]
fn text_only_turn_is_not_empty() {
    let seen = OutputSeen {
        text: true,
        tool_calls: false,
    };
    assert!(!seen.is_empty());
}

#[test]
fn describe_covers_every_combination() {
    let cases = [
        ((false, false), "none"),
        ((true, false), "text"),
        ((false, true), "tool_calls"),
        ((true, true), "text+tool_calls"),
    ];
    for ((text, tool_calls), expected) in cases {
        let seen = OutputSeen { text, tool_calls };
        assert_eq!(seen.describe(), expected);
    }
}
