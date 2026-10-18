//! Paste buffer regression retained from native input handling.
use super::fixture::paste;
use crate::attach::next_paste_id;
use crate::*;

#[test]
fn next_paste_id_starts_at_one_and_increments() {
    let mut pastes: Vec<PendingTextPaste> = Vec::new();
    assert_eq!(next_paste_id(&pastes), 1);
    pastes.push(paste(1, "a"));
    assert_eq!(next_paste_id(&pastes), 2);
    pastes.push(paste(2, "b"));
    assert_eq!(next_paste_id(&pastes), 3);
}
