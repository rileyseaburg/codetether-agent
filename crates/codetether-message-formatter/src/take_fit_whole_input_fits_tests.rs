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
fn take_fit_whole_input_fits() {
    let (taken, rest) = take_fit("short", 10, true);
    assert_eq!(taken, "short");
    assert!(rest.is_empty());
}
