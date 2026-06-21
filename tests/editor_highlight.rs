//! Tests for Rust syntax highlighting in the editor.

use codetether_agent::tui::ui::editor::helix_backend::HelixBackend;
use codetether_agent::tui::ui::editor::highlight::{capture_color, HighlightMap};
use codetether_agent::tui::ui::editor::EditorBackend;

#[test]
fn keyword_is_colored() {
    // "fn" is a keyword at bytes 0..2.
    let hl = HighlightMap::rust("fn main() {}\n");
    assert_eq!(hl.color_at(0), capture_color("keyword"));
    assert!(hl.color_at(0).is_some());
}

#[test]
fn plain_text_has_no_color() {
    let hl = HighlightMap::rust("");
    assert_eq!(hl.color_at(0), None);
}

#[test]
fn visible_cells_carry_colors() {
    let b = HelixBackend::from_str("fn main() {}\n");
    let line = &b.visible_lines(0, 1)[0];
    // First two cells ('f','n') are a keyword -> colored.
    assert!(line.cells[0].fg.is_some());
    assert_eq!(line.cells[0].fg, capture_color("keyword"));
}
