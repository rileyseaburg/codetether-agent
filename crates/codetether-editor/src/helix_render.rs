//! Converts rope lines into colored editor cells.
//!
//! [`colored_line`] walks a rope line, tracks each character's absolute byte
//! offset, and looks up its color in the document
//! [`HighlightMap`](super::highlight::HighlightMap). The trailing line ending is
//! dropped so the renderer never draws a stray newline.

use ropey::RopeSlice;

use super::backend::{EditorCell, EditorLine};
use super::highlight::HighlightMap;

/// Builds a colored [`EditorLine`] from a rope line starting at `line_byte`.
pub fn colored_line(line: RopeSlice<'_>, line_byte: usize, hl: &HighlightMap) -> EditorLine {
    let mut byte = line_byte;
    let mut cells = Vec::new();
    for ch in line.chars() {
        if ch != '\n' && ch != '\r' {
            cells.push(EditorCell {
                ch,
                fg: hl.color_at(byte),
            });
        }
        byte += ch.len_utf8();
    }
    EditorLine { cells }
}
