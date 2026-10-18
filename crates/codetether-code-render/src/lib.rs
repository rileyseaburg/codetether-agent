//! Terminal code boxes, Mermaid dispatch and unified-diff coloring.
//!
//! [render] frames a fenced block or renders supported Mermaid source.
//! [render_code_body] styles the body alone and [colorize_diff_line] supplies
//! the same diff colors to other terminal views.
mod block;
mod body;
mod diff;
mod header;
pub use block::render;
pub use body::render_code_body;
pub use diff::colorize_diff_line;
