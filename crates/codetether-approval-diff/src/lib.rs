//! Syntax-aware terminal rendering of approval unified diffs.
//!
//! lines maintains separate old/new syntax state and highlights additions,
//! deletions, context and metadata using lazily initialized syntax assets.
mod assets;
mod code;
mod diff;
mod highlight;
mod line;
mod syntax;
pub use diff::lines;
#[cfg(test)]
mod tests;
