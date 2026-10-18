//! Paste buffer regression retained from native input handling.
use super::fixture::paste;
use crate::*;

#[test]
fn expand_replaces_placeholder_with_full_content() {
    let pastes = vec![paste(1, "hello\nworld")];
    let prompt = "please review [Pasted text #1: 2 lines, 11 B] and respond";
    let out = expand_paste_placeholders(prompt, &pastes);
    assert!(out.contains("--- Begin pasted text #1 (2 lines, 11 bytes) ---"));
    assert!(out.contains("hello\nworld"));
    assert!(out.contains("--- End pasted text #1 ---"));
    assert!(out.starts_with("please review "));
    assert!(out.ends_with(" and respond"));
}
