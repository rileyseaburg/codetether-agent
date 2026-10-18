//! Cursor accessors for [`FileBuffer`], used by LSP go-to-definition.
//!
//! Kept separate from the open/save core so each file stays within the
//! 50-line budget and retains a single responsibility.

use super::backend::EditorBackend;
use super::file_buffer::FileBuffer;

impl FileBuffer {
    /// The current `(line, col)` cursor position (0-based).
    pub fn cursor(&self) -> (usize, usize) {
        self.backend().cursor()
    }

    /// Moves the cursor to `(line, col)`, clamped to the document.
    ///
    /// Does not mark the buffer dirty (cursor movement is not an edit).
    pub fn set_cursor(&mut self, line: usize, col: usize) {
        self.backend_for_cursor().set_cursor((line, col));
    }
}
