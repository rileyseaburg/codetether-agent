//! Paste buffer regression retained from native input handling.
use super::fixture::paste;
use crate::*;

#[test]
fn expand_drops_pastes_whose_placeholder_was_deleted() {
    let pastes = vec![paste(1, "hello"), paste(2, "world")];
    // Only #2's placeholder is present.
    let prompt = "look at [Pasted text #2: 1 lines, 5 B]";
    let out = expand_paste_placeholders(prompt, &pastes);
    assert!(out.contains("world"));
    assert!(!out.contains("hello"));
    assert!(!out.contains("#1"));
}
