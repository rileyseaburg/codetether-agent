//! Tests for alignment, diff coloring, and inline code in table cells.

use super::render::{align, cellstyle, inline};

#[test]
fn parses_alignment_markers() {
    let a = align::parse_aligns("| :--- | :--: | ---: |");
    assert_eq!(
        a,
        vec![
            align::Align::Left,
            align::Align::Center,
            align::Align::Right
        ]
    );
}

#[test]
fn right_aligns_text() {
    assert_eq!(align::align("12", 5, align::Align::Right), "   12");
    assert_eq!(align::align("ab", 6, align::Align::Center), "  ab  ");
}

#[test]
fn colors_diff_cells() {
    use ratatui::style::{Color, Style};
    let base = Style::default();
    assert_eq!(cellstyle::cell_style("+165", base).fg, Some(Color::Green));
    assert_eq!(cellstyle::cell_style("-127", base).fg, Some(Color::Red));
    assert_eq!(cellstyle::cell_style("+165/-127", base).fg, None);
    assert_eq!(cellstyle::cell_style("edited", base).fg, None);
}

#[test]
fn splits_inline_code_in_cell() {
    use ratatui::style::Style;
    let spans = inline::cell_spans("run `cargo build` now", Style::default());
    assert!(spans.len() >= 3);
    assert!(spans.iter().any(|s| s.content.contains("cargo build")));
}
