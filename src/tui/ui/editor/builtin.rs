//! Dependency-free editor backend.
//!
//! [`BuiltinBackend`] stores the document as a vector of line strings and
//! renders every character with the default theme color. It carries no
//! syntax highlighting; that is the job of the `helix` backend. This type is
//! always available so the editor view compiles and runs on the default
//! toolchain.

use super::backend::{EditorBackend, EditorCell, EditorLine};

/// An in-memory text buffer backed by one `String` per line.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::ui::editor::builtin::BuiltinBackend;
/// use codetether_agent::tui::ui::editor::EditorBackend;
///
/// let b = BuiltinBackend::from_str("fn main() {}\nlet x = 1;");
/// assert_eq!(b.line_count(), 2);
/// assert_eq!(b.visible_lines(1, 1)[0].cells.len(), 10);
/// ```
#[derive(Debug, Clone, Default)]
pub struct BuiltinBackend {
    lines: Vec<String>,
    cursor: (usize, usize),
}

impl BuiltinBackend {
    /// Builds a backend from the given source text, splitting on `'\n'`.
    pub fn from_str(text: &str) -> Self {
        let lines: Vec<String> = text.split('\n').map(str::to_string).collect();
        Self { lines, cursor: (0, 0) }
    }
}

impl EditorBackend for BuiltinBackend {
    fn name(&self) -> &'static str {
        "builtin"
    }

    fn line_count(&self) -> usize {
        self.lines.len()
    }

    fn visible_lines(&self, top: usize, height: usize) -> Vec<EditorLine> {
        self.lines
            .iter()
            .skip(top)
            .take(height)
            .map(|l| EditorLine {
                cells: l.chars().map(|ch| EditorCell { ch, fg: None }).collect(),
            })
            .collect()
    }

    fn cursor(&self) -> (usize, usize) {
        self.cursor
    }
}
