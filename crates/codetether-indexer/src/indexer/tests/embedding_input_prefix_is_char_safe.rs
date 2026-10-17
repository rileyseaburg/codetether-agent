//! Indexer behavior regression.

use super::*;

#[test]
fn embedding_input_prefix_is_char_safe() {
    let input = "✓✓✓hello";
    let prefixed = build_embedding_input("src/main.rs", "rust", input, 2);
    assert!(prefixed.contains("✓✓"));
}
