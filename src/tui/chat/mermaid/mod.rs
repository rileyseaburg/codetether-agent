//! # Mermaid Diagram Rendering
//!
//! Renders ```` ```mermaid ```` fenced blocks as Unicode box-drawing diagrams
//! inside the chat transcript, instead of dumping raw mermaid source.
//!
//! Supported: `flowchart`/`graph` (TD and LR) and `sequenceDiagram`.
//! Unsupported or malformed sources return `None` so the caller can fall
//! back to plain code-block rendering.
//!
//! ## Quick Start
//!
//! ```rust
//! use codetether_agent::tui::chat::mermaid::render_block;
//!
//! let lines = render_block("flowchart TD\n  A[Start] --> B[Done]", 60)
//!     .expect("valid flowchart");
//! assert!(lines.len() > 2);
//! ```

use ratatui::text::Line;

pub mod canvas;
mod canvas_lines;
pub mod frame;
pub mod model;
pub mod parse;
pub mod render;

#[cfg(test)]
mod tests;

/// Detect whether a fence language marks a mermaid block.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::is_mermaid;
///
/// assert!(is_mermaid("mermaid"));
/// assert!(is_mermaid("Mermaid"));
/// assert!(!is_mermaid("rust"));
/// ```
pub fn is_mermaid(language: &str) -> bool {
    language.trim().eq_ignore_ascii_case("mermaid")
}

/// Parse and render a mermaid block, framed to `width` columns.
///
/// # Returns
///
/// `Some(lines)` when the source parses into a supported diagram, otherwise
/// `None` so the caller renders the source as an ordinary code block.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::render_block;
///
/// assert!(render_block("not a diagram", 40).is_none());
/// assert!(render_block("sequenceDiagram\n A->>B: hi", 40).is_some());
/// ```
pub fn render_block(source: &str, width: usize) -> Option<Vec<Line<'static>>> {
    let diagram = parse::parse(source)?;
    let body = render::render(&diagram, frame::BORDER);
    Some(frame::frame(body, width))
}
