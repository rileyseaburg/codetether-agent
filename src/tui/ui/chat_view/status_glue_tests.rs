//! Narrow-terminal regression tests for status-bar hint packing.

use super::compact_hints::compact_keybinding_spans;
use super::status_pack::pack_spans;
use ratatui::text::Line;

fn text(line: &Line<'_>) -> String {
    line.spans.iter().map(|span| span.content.as_ref()).collect()
}

/// iPhone portrait SSH clients render around 46 columns; a key must never be
/// split from its label (previously `... | Esc` / `: Back | ...`).
#[test]
fn phone_width_never_splits_key_from_label() {
    for width in 20..=60 {
        let lines = pack_spans(compact_keybinding_spans(), width);
        for line in &lines {
            let rendered = text(line);
            assert!(
                !rendered.starts_with(':'),
                "width {width}: line starts with a dangling label: {rendered:?}"
            );
        }
        let joined: String = lines.iter().map(text).collect();
        assert!(joined.contains("Esc: Back"), "width {width}: {joined:?}");
    }
}

#[test]
fn wide_terminal_keeps_hints_on_one_line() {
    assert_eq!(pack_spans(compact_keybinding_spans(), 120).len(), 1);
}
