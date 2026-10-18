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
fn inline_math_styled_separately() {
    let f = MessageFormatter::new(80);
    let content = "Let \\(x \\in \\mathbb C\\) be a number.";
    let lines = f.format_content(content, "assistant");
    // Inline math should produce a magenta italic span containing prettified math.
    let mut found = false;
    for line in &lines {
        for span in &line.spans {
            if span.content.contains("ℂ")
                && span.style.fg == Some(Color::Magenta)
                && span.style.add_modifier.contains(Modifier::ITALIC)
            {
                found = true;
            }
        }
    }
    assert!(found, "expected styled inline-math span with ℂ glyph");
}
