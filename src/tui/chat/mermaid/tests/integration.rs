//! Tests for mermaid detection and MessageFormatter integration.

use super::flatten::flatten;
use crate::tui::chat::mermaid::{is_mermaid, render_block};
use crate::tui::message_formatter::MessageFormatter;

#[test]
fn language_detection() {
    assert!(is_mermaid(" MERMAID "));
    assert!(!is_mermaid("rust"));
}

#[test]
fn unsupported_source_is_rejected() {
    assert!(render_block("not a diagram", 40).is_none());
}

#[test]
fn formatter_renders_mermaid_fence_as_diagram() {
    let fmt = MessageFormatter::new(60);
    let out = fmt.format_content(
        "```mermaid\nflowchart TD\n A[Go] --> B[Stop]\n```",
        "assistant",
    );
    let body = flatten(&out);
    assert!(body.contains("┌─ Mermaid ─"));
    assert!(!body.contains("flowchart TD"));
}

#[test]
fn formatter_falls_back_for_invalid_mermaid() {
    let fmt = MessageFormatter::new(60);
    let body = flatten(&fmt.format_content("```mermaid\nnot a diagram\n```", "assistant"));
    assert!(body.contains("not a diagram"));
    assert!(!body.contains("┌─ Mermaid ─"));
}

#[test]
fn formatter_leaves_other_languages_alone() {
    let fmt = MessageFormatter::new(60);
    let body = flatten(&fmt.format_content("```rust\nlet x = 1;\n```", "assistant"));
    assert!(body.contains("let x = 1;"));
    assert!(!body.contains("Mermaid"));
}
