//! [`EditorEdit`] implementation for the ropey-backed [`HelixBackend`].
//!
//! Cursor `(line, col)` positions are translated to absolute rope char
//! indices for each operation, then back, so edits stay O(log n) on the rope.
//!
//! # Examples
//!
//! ```
//! use codetether_agent::tui::ui::editor::helix_backend::HelixBackend;
//! use codetether_agent::tui::ui::editor::{EditorBackend, EditorEdit, Move};
//!
//! let mut b = HelixBackend::from_str("ab\n");
//! b.insert_char('X');           // "Xab"
//! assert_eq!(b.cursor(), (0, 1));
//! b.move_cursor(Move::Right);   // after 'a'
//! b.delete_back();              // removes 'a' -> "Xb"
//! assert_eq!(b.to_text(), "Xb\n");
//! ```

use super::edit::{EditorEdit, Move};
use super::helix_backend::HelixBackend;

impl EditorEdit for HelixBackend {
    fn insert_char(&mut self, ch: char) {
        let idx = self.cursor_char_idx();
        self.rope_mut().insert_char(idx, ch);
        if ch == '\n' {
            self.set_cursor((self.cursor.0 + 1, 0));
        } else {
            let (l, c) = self.cursor;
            self.set_cursor((l, c + 1));
        }
    }

    fn delete_back(&mut self) {
        let idx = self.cursor_char_idx();
        if idx == 0 {
            return;
        }
        self.move_cursor(Move::Left);
        self.rope_mut().remove(idx - 1..idx);
    }

    fn delete_forward(&mut self) {
        let idx = self.cursor_char_idx();
        if idx < self.rope().len_chars() {
            self.rope_mut().remove(idx..idx + 1);
        }
    }

    fn move_cursor(&mut self, dir: Move) {
        let idx = self.cursor_char_idx();
        let next = match dir {
            Move::Left => idx.saturating_sub(1),
            Move::Right => (idx + 1).min(self.rope().len_chars()),
            Move::Up | Move::Down => self.vertical_idx(dir),
            Move::LineStart => self.rope().line_to_char(self.cursor.0),
            Move::LineEnd => self.line_end_idx(),
        };
        self.set_cursor_from_idx(next);
    }

    fn to_text(&self) -> String {
        self.rope().to_string()
    }
}
