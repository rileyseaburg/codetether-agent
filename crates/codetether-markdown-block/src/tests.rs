//! Tests for block-level markdown classification.

use super::detect::{Block, classify};

#[test]
fn header_is_styled_block() {
    assert!(matches!(classify("# Title"), Block::Styled(_)));
    assert!(matches!(classify("### Sub"), Block::Styled(_)));
}

#[test]
fn bullet_and_numbered_lists_prefixed() {
    assert!(matches!(classify("- item"), Block::Prefixed { .. }));
    assert!(matches!(classify("1. item"), Block::Prefixed { .. }));
}

#[test]
fn rule_and_quote_detected() {
    assert!(matches!(classify("---"), Block::Styled(_)));
    assert!(matches!(classify("> quote"), Block::Prefixed { .. }));
}

#[test]
fn plain_text_is_plain() {
    assert!(matches!(classify("just text"), Block::Plain));
    assert!(matches!(classify("#nospace"), Block::Plain));
}
