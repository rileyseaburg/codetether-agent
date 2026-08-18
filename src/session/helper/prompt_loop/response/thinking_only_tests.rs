//! Tests for the thinking-only assistant-turn guard.

use super::is_thinking_only;
use crate::provider::ContentPart;

fn thinking(text: &str) -> ContentPart {
    ContentPart::Thinking {
        text: text.to_string(),
        signature: None,
    }
}

fn text(text: &str) -> ContentPart {
    ContentPart::Text {
        text: text.to_string(),
    }
}

#[test]
fn detects_thinking_only_turn() {
    assert!(is_thinking_only(&[thinking("reasoning")]));
}

#[test]
fn ignores_blank_text_alongside_thinking() {
    assert!(is_thinking_only(&[thinking("reasoning"), text("   ")]));
}

#[test]
fn rejects_turn_with_visible_text() {
    assert!(!is_thinking_only(&[thinking("reasoning"), text("answer")]));
}

#[test]
fn rejects_turn_without_any_thinking() {
    assert!(!is_thinking_only(&[text("answer")]));
    assert!(!is_thinking_only(&[]));
}
