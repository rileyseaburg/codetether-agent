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
fn test_code_block_detection() {
    let formatter = MessageFormatter::new(80);
    let content = "```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
    let lines = formatter.format_content(content, "assistant");
    assert!(!lines.is_empty());
}
