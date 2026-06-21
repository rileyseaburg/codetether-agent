//! Ropey-powered editor backend.
//!
//! [`HelixBackend`] stores the document in a [`ropey::Rope`], the same
//! battle-tested rope data structure the Helix editor uses (Helix re-exports
//! it as `helix_core::Rope`). Compared to the line-vector
//! [`BuiltinBackend`](super::builtin::BuiltinBackend) this gives efficient
//! inserts/deletes and correct char/line indexing on large files.
//!
//! Syntax highlighting (tree-sitter) is not wired in yet, so cells are emitted
//! with the theme default color; that is the next increment.

use ropey::{Rope, RopeSlice};

use super::backend::{EditorBackend, EditorCell, EditorLine};

/// A rope-backed document using Helix's core text representation.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::ui::editor::helix_backend::HelixBackend;
/// use codetether_agent::tui::ui::editor::EditorBackend;
///
/// let b = HelixBackend::from_str("fn main() {}\nlet x = 1;\n");
/// assert_eq!(b.name(), "helix");
/// assert_eq!(b.visible_lines(0, 1)[0].cells.len(), 12);
/// ```
#[derive(Debug, Clone)]
pub struct HelixBackend {
    rope: Rope,
    cursor: (usize, usize),
}

impl HelixBackend {
    /// Builds a backend from source text.
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            cursor: (0, 0),
        }
    }

    /// Borrows the underlying rope (for future syntax/edit operations).
    pub fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl EditorBackend for HelixBackend {
    fn name(&self) -> &'static str {
        "helix"
    }

    fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    fn visible_lines(&self, top: usize, height: usize) -> Vec<EditorLine> {
        (top..self.rope.len_lines().min(top + height))
            .map(|i| line_cells(self.rope.line(i)))
            .collect()
    }

    fn cursor(&self) -> (usize, usize) {
        self.cursor
    }
}

/// Converts a rope line into styled cells, dropping the trailing line ending.
fn line_cells(line: RopeSlice<'_>) -> EditorLine {
    let cells = line
        .chars()
        .filter(|c| *c != '\n' && *c != '\r')
        .map(|ch| EditorCell { ch, fg: None })
        .collect();
    EditorLine { cells }
}