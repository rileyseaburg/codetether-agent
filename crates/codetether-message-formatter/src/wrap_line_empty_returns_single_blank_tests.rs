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
fn wrap_line_empty_returns_single_blank() {
    let f = MessageFormatter::new(20);
    let out = f.wrap_line(vec![], 16);
    assert_eq!(out.len(), 1);
}
