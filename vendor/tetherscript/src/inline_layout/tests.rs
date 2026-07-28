use super::*;

fn texts(line: &LineBox) -> Vec<String> {
    line.children
        .iter()
        .filter_map(|c| match &c.inline_box.kind {
            InlineBoxKind::Text(r) => Some(r.text.clone()),
            _ => None,
        })
        .collect()
}

#[test]
fn measures_text_as_monospace() {
    assert_eq!(measure_text("abcd"), 32.0);
    assert_eq!(measure_text(""), 0.0);
}

#[test]
fn single_line_when_content_fits() {
    let lines = layout_inline(&[InlineBox::text("hello")], 80.0);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].width, 40.0);
    assert_eq!(texts(&lines[0]), vec!["hello"]);
}

#[test]
fn wraps_at_word_boundary() {
    let lines = layout_inline(&[InlineBox::text("hello world")], 48.0);
    assert_eq!(lines.len(), 2);
    assert_eq!(texts(&lines[0]), vec!["hello"]);
    assert_eq!(texts(&lines[1]), vec!["world"]);
}

#[test]
fn breaks_mid_word_for_overflowing_word() {
    let lines = layout_inline(&[InlineBox::text("abcdefghij")], 32.0);
    assert_eq!(lines.len(), 3);
    assert_eq!(texts(&lines[0]), vec!["abcd"]);
    assert_eq!(texts(&lines[1]), vec!["efgh"]);
    assert_eq!(texts(&lines[2]), vec!["ij"]);
}

#[test]
fn explicit_line_break_creates_new_line() {
    let lines = layout_inline(&[InlineBox::text("a\nb")], 200.0);
    assert_eq!(lines.len(), 2);
    assert_eq!(texts(&lines[0]), vec!["a"]);
    assert_eq!(texts(&lines[1]), vec!["b"]);
}

#[test]
fn br_element_forces_break() {
    let lines = layout_inline(
        &[InlineBox::text("a"), InlineBox::br(), InlineBox::text("b")],
        200.0,
    );
    assert_eq!(lines.len(), 2);
    assert_eq!(texts(&lines[0]), vec!["a"]);
    assert_eq!(texts(&lines[1]), vec!["b"]);
}

#[test]
fn atomic_inline_wraps_and_positions() {
    let lines = layout_inline(
        &[InlineBox::text("abcd"), InlineBox::image(40.0, 20.0)],
        64.0,
    );
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].children[0].x, 0.0);
    assert_eq!(lines[1].children[0].width, 40.0);
}

#[test]
fn baseline_aligns_mixed_content() {
    let lines = layout_inline(
        &[InlineBox::text("x"), InlineBox::inline_block(10.0, 30.0)],
        100.0,
    );
    assert_eq!(lines.len(), 1);
    assert!(lines[0].baseline >= 30.0);
    assert!(lines[0].height >= 33.2);
}

#[test]
fn vertical_align_top_middle_bottom() {
    let boxes = vec![
        InlineBox::inline_block(10.0, 20.0).with_vertical_align(VerticalAlign::Top),
        InlineBox::inline_block(10.0, 10.0).with_vertical_align(VerticalAlign::Middle),
        InlineBox::inline_block(10.0, 5.0).with_vertical_align(VerticalAlign::Bottom),
    ];
    let lines = layout_inline(&boxes, 100.0);
    assert_eq!(lines[0].children[0].y, 0.0);
    assert!(lines[0].children[1].y > 0.0);
    assert!(lines[0].children[2].y > lines[0].children[1].y);
}

#[test]
fn break_before_atomic_when_it_does_not_fit() {
    let lines = layout_inline(
        &[InlineBox::text("abc"), InlineBox::inline_block(32.0, 10.0)],
        48.0,
    );
    assert_eq!(lines.len(), 2);
    assert_eq!(texts(&lines[0]), vec!["abc"]);
    assert_eq!(lines[1].children[0].x, 0.0);
}
