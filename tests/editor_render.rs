//! Tests for editor line rendering (gutter + text).

use codetether_agent::tui::ui::editor::editor_lines;
use codetether_agent::tui::ui::editor::helix_backend::HelixBackend;

#[test]
fn renders_gutter_and_text() {
    let b = HelixBackend::from_str("alpha\nbeta\ngamma\n");
    let lines = editor_lines(&b, 0, 3, 0);
    assert_eq!(lines.len(), 3);

    // First visual line: gutter "1 " then "alpha".
    let spans = &lines[0].spans;
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].content.as_ref(), "1 ");
    assert_eq!(spans[1].content.as_ref(), "alpha");
}

#[test]
fn gutter_aligns_to_widest_line_number() {
    // 12 lines -> 2-digit gutter, so line 1 is right-padded to "1".
    let text = (1..=12).map(|n| n.to_string()).collect::<Vec<_>>().join("\n");
    let b = HelixBackend::from_str(&text);
    let lines = editor_lines(&b, 0, 1, 0);
    assert_eq!(lines[0].spans[0].content.as_ref(), " 1 ");
}
