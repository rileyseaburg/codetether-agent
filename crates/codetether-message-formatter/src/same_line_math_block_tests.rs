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
fn same_line_math_block() {
    let f = MessageFormatter::new(40);
    let content = "\\[ x = 1 \\]";
    let lines = f.format_content(content, "assistant");
    let rendered: Vec<String> = lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>()
        })
        .collect();
    assert!(rendered.iter().any(|l| l.contains("Math")));
    assert!(rendered.iter().any(|l| l.contains("x = 1")));
}
