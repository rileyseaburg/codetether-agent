//! Inline layout unit tests.

use super::*;
use std::collections::HashMap;

fn text(content: &str, size: i64) -> InlineFragment {
    let width = measure_text_width(content, size);
    InlineFragment::new(
        width,
        size * 6 / 5,
        size,
        FragmentKind::Text {
            content: content.to_string(),
            font_size: size,
        },
    )
}

#[test]
fn wraps_text_at_word_boundaries() {
    let lines = break_lines(&[text("hello world", 10)], 36);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].fragments.len(), 2);
    assert_eq!(lines[1].fragments.len(), 1);
    assert_eq!(lines[1].fragments[0].x, 0);
}

#[test]
fn allows_single_word_overflow() {
    let lines = break_lines(&[text("superlong", 10)], 20);
    assert_eq!(lines.len(), 1);
    assert!(lines[0].fragments[0].width > 20);
}

#[test]
fn aligns_fragments_to_shared_baseline() {
    let mut line = LineBox::new(10, 100);
    line.push(text("big", 20));
    line.push(text("x", 10));
    let line = line.finalize();
    assert_eq!(line.baseline, 20);
    assert_eq!(line.fragments[0].y, 10);
    assert_eq!(line.fragments[1].y, 20);
}

#[test]
fn measures_line_height_from_styles() {
    let mut styles = HashMap::new();
    styles.insert("font-size".to_string(), "20px".to_string());
    styles.insert("line-height".to_string(), "1.5".to_string());
    assert_eq!(measure_line_height(&styles), 30);
}
