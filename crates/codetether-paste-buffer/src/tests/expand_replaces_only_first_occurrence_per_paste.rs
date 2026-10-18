//! Paste buffer regression retained from native input handling.
use super::fixture::paste;
use crate::*;

#[test]
fn expand_replaces_only_first_occurrence_per_paste() {
    // If the user typed the same placeholder string twice, only the
    // first match expands so the sidecar content is never duplicated.
    let pastes = vec![paste(1, "DATA")];
    let prompt = "[Pasted text #1: 1 lines, 4 B] then again [Pasted text #1: 1 lines, 4 B]";
    let out = expand_paste_placeholders(prompt, &pastes);
    assert_eq!(out.matches("DATA").count(), 1);
    assert_eq!(out.matches("[Pasted text #1: 1 lines, 4 B]").count(), 1);
}
