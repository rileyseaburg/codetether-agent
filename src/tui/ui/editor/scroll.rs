//! Keeps the editor scroll offset tracking the cursor.
//!
//! [`follow_cursor`] returns the new top-line offset so the cursor row stays
//! within the visible window of `height` rows. Pure arithmetic, so it is fully
//! unit-testable without a terminal.

use super::backend::EditorBackend;

/// Returns the scroll offset that keeps the cursor visible.
///
/// If the cursor is above `scroll`, the window moves up to the cursor row. If
/// it is at or below `scroll + height`, the window moves down so the cursor is
/// the last visible row.
pub fn follow_cursor<B: EditorBackend>(backend: &B, scroll: usize, height: usize) -> usize {
    if height == 0 {
        return scroll;
    }
    let (line, _) = backend.cursor();
    if line < scroll {
        line
    } else if line >= scroll + height {
        line + 1 - height
    } else {
        scroll
    }
}
