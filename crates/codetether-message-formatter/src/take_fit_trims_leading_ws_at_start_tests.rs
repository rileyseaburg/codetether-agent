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
fn take_fit_trims_leading_ws_at_start() {
    let (taken, rest) = take_fit("   hello", 8, true);
    assert_eq!(taken, "hello");
    assert!(rest.is_empty());
}
