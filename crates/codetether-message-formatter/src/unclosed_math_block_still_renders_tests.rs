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
fn unclosed_math_block_still_renders() {
    let f = MessageFormatter::new(40);
    let content = "\\[\nx = 1";
    let lines = f.format_content(content, "assistant");
    // Should not panic and should produce some output containing the content.
    let rendered: String = lines
        .iter()
        .flat_map(|l| l.spans.iter().map(|s| s.content.as_ref()))
        .collect();
    assert!(rendered.contains("x = 1"));
}
