use std::mem::size_of;

use ratatui::text::{Line, Span};

use super::weight;

#[test]
fn rendered_weight_counts_structures_and_text() {
    let lines = vec![Line::from("hello")];
    let minimum = size_of::<Line<'static>>() + size_of::<Span<'static>>() + 5;
    assert!(weight(&lines, lines.capacity()) >= minimum);
}
