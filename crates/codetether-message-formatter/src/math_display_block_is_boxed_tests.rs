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
fn math_display_block_is_boxed() {
    let f = MessageFormatter::new(40);
    let content = "Therefore:\n\\[\nP_sP_t=\\delta_{st}P_s\n\\]\nDone.";
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
    // Header, body, footer present.
    assert!(rendered.iter().any(|l| l.contains("Math")));
    assert!(rendered.iter().any(|l| l.contains("δ")));
    assert!(rendered.iter().any(|l| l.starts_with("└")));
}
