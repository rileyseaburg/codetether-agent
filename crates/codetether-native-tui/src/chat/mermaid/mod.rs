//! Mermaid parsing and terminal rendering supplied by the shared crate.
pub use codetether_mermaid::{canvas, frame, is_mermaid, model, parse, render, render_block};
#[cfg(test)]
mod tests;
