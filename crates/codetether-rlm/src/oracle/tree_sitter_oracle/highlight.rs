//! Rust syntax captures backed by the oracle's retained parse tree.

use anyhow::{Context, Result};
use streaming_iterator::StreamingIterator;

use super::{AstCapture, TreeSitterOracle};

impl TreeSitterOracle {
    /// Returns the Rust highlight-query captures for the oracle source.
    ///
    /// # Returns
    ///
    /// One record per capture, preserving its name and UTF-8 byte range.
    ///
    /// # Errors
    ///
    /// Returns an error if the source or bundled Rust highlight query cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_rlm::TreeSitterOracle;
    /// let mut oracle = TreeSitterOracle::new("fn main() {}".to_string());
    /// let captures = oracle.rust_highlight_captures().unwrap();
    /// assert!(!captures.is_empty());
    /// ```
    pub fn rust_highlight_captures(&mut self) -> Result<Vec<AstCapture>> {
        self.parse()?;
        let tree = self
            .tree
            .as_ref()
            .context("tree-sitter source was not parsed")?;
        let language = tree_sitter_rust::LANGUAGE.into();
        let query = tree_sitter::Query::new(&language, tree_sitter_rust::HIGHLIGHTS_QUERY)?;
        Ok(collect(&query, tree.root_node(), self.source.as_bytes()))
    }
}

fn collect(
    query: &tree_sitter::Query,
    root: tree_sitter::Node<'_>,
    source: &[u8],
) -> Vec<AstCapture> {
    let names = query.capture_names();
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(query, root, source);
    let mut captures = Vec::new();
    while let Some(match_) = matches.next() {
        for capture in match_.captures {
            let range = capture.node.byte_range();
            captures.push(AstCapture {
                name: names[capture.index as usize].to_string(),
                start_byte: range.start,
                end_byte: range.end,
            });
        }
    }
    captures
}

#[cfg(test)]
#[path = "highlight_tests.rs"]
mod tests;
