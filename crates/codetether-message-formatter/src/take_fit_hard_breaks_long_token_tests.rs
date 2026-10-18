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
fn take_fit_hard_breaks_long_token() {
    let (taken, rest) = take_fit("abcdefghij", 4, true);
    assert_eq!(taken, "abcd");
    assert_eq!(rest, "efghij");
}
