//! Editor backend abstraction.
//!
//! The renderer asks a backend for visible lines and the cursor position; it
//! never reaches into the backend's internal representation. This is the seam
//! that lets the Helix-core engine slot in behind a cargo feature without the
//! renderer ever referencing `helix-core` types.

/// A single styled cell of editor text.
///
/// Colors are optional 24-bit RGB triples; `None` means "use the theme
/// default". Keeping styling as plain data avoids leaking any backend's
/// theme types across the seam.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorCell {
    /// The grapheme/character displayed in this cell.
    pub ch: char,
    /// Optional foreground color as `(r, g, b)`.
    pub fg: Option<(u8, u8, u8)>,
}

/// A logical line of editor text, already resolved to styled cells.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EditorLine {
    /// The styled cells making up this line, left to right.
    pub cells: Vec<EditorCell>,
}

/// An editing engine that can supply rendered lines and a cursor.
///
/// Implementors own the document state; the renderer is purely a consumer.
pub trait EditorBackend {
    /// Human-readable name of this backend (e.g. `"builtin"` or `"helix"`).
    fn name(&self) -> &'static str;

    /// Total number of logical lines in the document.
    fn line_count(&self) -> usize;

    /// Returns up to `height` lines starting at logical line `top`.
    ///
    /// Lines past the end of the document are omitted (the renderer pads).
    fn visible_lines(&self, top: usize, height: usize) -> Vec<EditorLine>;

    /// Cursor position as `(line, column)`, both zero-based.
    fn cursor(&self) -> (usize, usize);
}
