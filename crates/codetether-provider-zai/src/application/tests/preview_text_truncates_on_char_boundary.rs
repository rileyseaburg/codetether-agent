//! Migrated application provider regression.
use super::*;
#[test]
fn preview_text_truncates_on_char_boundary() {
    let text = "a😀b";
    assert_eq!(ZaiProvider::preview_text(text, 2), "a😀");
}
