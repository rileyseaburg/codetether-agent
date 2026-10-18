use ratatui::text::Line;

use super::visible_lines;

fn lines(count: usize) -> Vec<Line<'static>> {
    (0..count)
        .map(|index| Line::from(index.to_string()))
        .collect()
}

#[test]
fn selects_only_interior_rows() {
    let lines = lines(100);
    let visible = visible_lines(&lines, 40, 10);

    assert_eq!(visible.len(), 8);
    assert_eq!(visible[0].spans[0].content, "40");
    assert_eq!(visible[7].spans[0].content, "47");
}

#[test]
fn clamps_at_buffer_tail() {
    let lines = lines(5);
    assert_eq!(visible_lines(&lines, 4, 10).len(), 1);
    assert!(visible_lines(&lines, 99, 10).is_empty());
}
