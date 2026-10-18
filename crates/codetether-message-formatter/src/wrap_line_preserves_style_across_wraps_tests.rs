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
fn wrap_line_preserves_style_across_wraps() {
    let f = MessageFormatter::new(20);
    let styled = Style::default().add_modifier(Modifier::BOLD);
    let spans = vec![Span::styled("alpha beta gamma delta", styled)];
    let out = f.wrap_line(spans, 10);
    for line in &out {
        for span in &line.spans {
            assert_eq!(span.style, styled);
        }
    }
}
