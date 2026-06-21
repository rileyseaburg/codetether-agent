//! Editing operations for editor backends.
//!
//! [`EditorEdit`] is intentionally separate from
//! [`EditorBackend`](super::backend::EditorBackend): rendering (read) and
//! mutation (write) are distinct responsibilities, so a backend can be a
//! pure viewer or a full editor. All positions are zero-based `(line, col)`.

/// Direction for cursor movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    /// One column left (wraps to previous line end).
    Left,
    /// One column right (wraps to next line start).
    Right,
    /// One line up, keeping column where possible.
    Up,
    /// One line down, keeping column where possible.
    Down,
}

/// Mutating operations layered on top of a render backend.
pub trait EditorEdit {
    /// Inserts `ch` at the cursor, advancing the cursor past it.
    fn insert_char(&mut self, ch: char);

    /// Deletes the character before the cursor (Backspace).
    fn delete_back(&mut self);

    /// Moves the cursor in the given direction, clamped to the document.
    fn move_cursor(&mut self, dir: Move);

    /// Renders the full document back to a `String`.
    fn to_text(&self) -> String;
}
