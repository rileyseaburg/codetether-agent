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
fn wrap_line_width_zero_is_noop() {
    let f = MessageFormatter::new(20);
    let spans = vec![Span::raw("anything")];
    let out = f.wrap_line(spans, 0);
    assert_eq!(out.len(), 1);
}
