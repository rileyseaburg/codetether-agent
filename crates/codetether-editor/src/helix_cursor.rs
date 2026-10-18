//! Cursor/char-index conversion helpers for [`HelixBackend`].
//!
//! Ropey addresses text by absolute char index, while the editor and renderer
//! think in `(line, col)`. These helpers translate between the two and keep
//! the cursor clamped to valid positions.

use super::edit::Move;
use super::helix_backend::HelixBackend;
use ropey::Rope;

impl HelixBackend {
    /// Mutable access to the underlying rope.
    pub(super) fn rope_mut(&mut self) -> &mut Rope {
        &mut self.rope
    }

    /// Overwrites the cursor position (no clamping).
    pub(super) fn set_cursor(&mut self, pos: (usize, usize)) {
        self.cursor = pos;
    }

    /// Absolute rope char index of the current cursor.
    pub(super) fn cursor_char_idx(&self) -> usize {
        let (line, col) = self.cursor;
        let line = line.min(self.rope.len_lines().saturating_sub(1));
        let line_start = self.rope.line_to_char(line);
        let line_len = self.rope.line(line).len_chars();
        line_start + col.min(line_len)
    }

    /// Sets the cursor from an absolute char index.
    pub(super) fn set_cursor_from_idx(&mut self, idx: usize) {
        let idx = idx.min(self.rope.len_chars());
        let line = self.rope.char_to_line(idx);
        let col = idx - self.rope.line_to_char(line);
        self.cursor = (line, col);
    }

    /// Char index after moving up/down one line, preserving column.
    pub(super) fn vertical_idx(&self, dir: Move) -> usize {
        let (line, col) = self.cursor;
        let target = match dir {
            Move::Up => line.saturating_sub(1),
            _ => (line + 1).min(self.rope.len_lines().saturating_sub(1)),
        };
        let start = self.rope.line_to_char(target);
        let len = self.rope.line(target).len_chars();
        start + col.min(len)
    }

    /// Char index of the end of the current line, before any trailing newline.
    pub(super) fn line_end_idx(&self) -> usize {
        let line = self.cursor.0.min(self.rope.len_lines().saturating_sub(1));
        let start = self.rope.line_to_char(line);
        let slice = self.rope.line(line);
        let len = slice.len_chars();
        let trailing_newline = len > 0 && slice.char(len - 1) == '\n';
        start + if trailing_newline { len - 1 } else { len }
    }
}
