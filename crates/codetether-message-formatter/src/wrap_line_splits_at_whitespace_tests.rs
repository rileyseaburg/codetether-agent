//! Existing formatter regression coverage.
#[allow(unused_imports)]
use crate::MessageFormatter;
#[allow(unused_imports)]
use crate::fit::take_fit;
#[allow(unused_imports)]
use codetether_math_text::prettify_math;
#[allow(unused_imports)]
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

#[test]
fn wrap_line_splits_at_whitespace() {
    let f = MessageFormatter::new(20);
    let spans = vec![Span::raw("hello world foo bar")];
    let out = f.wrap_line(spans, 10);
    assert!(out.len() >= 2);
    for line in &out {
        assert!(line.width() <= 10, "line too wide: {}", line.width());
    }
}
