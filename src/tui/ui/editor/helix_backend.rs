//! Ropey-powered editor backend.
//!
//! [`HelixBackend`] stores the document in a [`ropey::Rope`] and a precomputed
//! [`HighlightMap`](super::highlight::HighlightMap) so rendered cells carry
//! tree-sitter syntax colors. Compared to the line-vector
//! [`BuiltinBackend`](super::builtin::BuiltinBackend) the rope gives efficient
//! inserts/deletes and correct char/line indexing on large files.

use ropey::Rope;

use super::backend::{EditorBackend, EditorLine};
use super::helix_render::colored_line;
use super::highlight::HighlightMap;

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
    pub(super) rope: Rope,
    pub(super) cursor: (usize, usize),
    pub(super) highlight: HighlightMap,
}

impl HelixBackend {
    /// Builds a backend from source text, computing initial highlights.
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            cursor: (0, 0),
            highlight: HighlightMap::rust(text),
        }
    }

    /// Borrows the underlying rope.
    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Recomputes syntax highlights from the current rope contents.
    pub(super) fn rebuild_highlight(&mut self) {
        self.highlight = HighlightMap::rust(&self.rope.to_string());
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
            .map(|i| {
                let byte = self.rope.line_to_byte(i);
                colored_line(self.rope.line(i), byte, &self.highlight)
            })
            .collect()
    }

    fn cursor(&self) -> (usize, usize) {
        self.cursor
    }
}
