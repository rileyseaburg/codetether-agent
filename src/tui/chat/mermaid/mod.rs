//! Mermaid parsing and terminal rendering supplied by the shared crate.
pub use codetether_mermaid::{canvas, frame, model, parse, render, is_mermaid, render_block};
#[cfg(test)]
mod tests;
