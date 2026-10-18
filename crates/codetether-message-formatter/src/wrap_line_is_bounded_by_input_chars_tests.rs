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
fn wrap_line_is_bounded_by_input_chars() {
    // Regression + structural guarantee: a whitespace-only span used to
    // loop forever (growing `out` until a ~48 GiB abort). Every produced
    // row now consumes >= 1 input char, so row count <= input char count.
    let f = MessageFormatter::new(20);
    assert!(f.wrap_line(vec![Span::raw("        ")], 4).len() <= 2);
    let input = "a   b   c   d   e   f";
    let out = f.wrap_line(vec![Span::raw(input)], 3);
    assert!(out.len() <= input.chars().count() && out.iter().all(|l| l.width() <= 3));
}
