//! Stateful Markdown message formatting for terminal chat views.
//!
//! [MessageFormatter] parses code, math, tables and prose in source order,
//! styles inline text and wraps it into owned ratatui lines.
mod block;
mod code;
mod code_content;
mod code_fence;
mod construction;
mod content;
mod content_finish;
mod content_line;
mod content_state;
mod fit;
mod image;
mod inline;
mod inline_bold;
mod inline_code;
mod inline_finish;
mod inline_italic;
mod inline_math;
mod inline_state;
mod math;
mod math_active;
mod math_open;
mod table;
mod wrap;
mod wrap_span;
mod wrap_state;
/// Fenced-block rendering compatibility module.
pub mod message_formatter_code_block {
    pub use codetether_code_render::render;
}
/// Code-body rendering compatibility module.
pub mod message_formatter_code_render {
    pub use codetether_code_render::render_code_body;
}
/// Formatter configured with a maximum terminal width.
/// # Examples
/// ```
/// let formatter = codetether_message_formatter::MessageFormatter::new(80);
/// assert_eq!(formatter.max_width(), 80);
/// ```
pub struct MessageFormatter {
    max_width: usize,
}
#[cfg(test)]
mod tests;
