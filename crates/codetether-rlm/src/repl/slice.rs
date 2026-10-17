//! Unicode character slicing and line chunking.
use super::*;
impl RlmRepl {
    /// Slice context by character positions
    ///
    /// # Arguments
    /// * `start` - Inclusive Unicode character offset.
    /// * `end` - Exclusive Unicode character offset, clamped to context length.
    /// # Returns
    /// A borrowed context slice; start is clamped to end.
    /// # Examples
    /// ```
    /// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
    /// let mut repl = RlmRepl::new("first\nsecond".into(), ReplRuntime::Rust);
    /// assert_eq!(repl.slice(0, 5), "first");
    /// ```
    pub fn slice(&self, start: usize, end: usize) -> &str {
        let total_chars = self.context.chars().count();
        let end = end.min(total_chars);
        let start = start.min(end);
        let start_byte = char_index_to_byte_index(&self.context, start);
        let end_byte = char_index_to_byte_index(&self.context, end);
        &self.context[start_byte..end_byte]
    }

    /// Split context into n chunks
    ///
    /// # Arguments
    /// * `n` - Requested chunk count; zero returns the whole context.
    /// # Returns
    /// Owned chunks made from contiguous context lines.
    /// # Examples
    /// ```
    /// use codetether_rlm::repl::{ReplRuntime, RlmRepl};
    /// let mut repl = RlmRepl::new("first\nsecond".into(), ReplRuntime::Rust);
    /// assert_eq!(repl.chunks(2).len(), 2);
    /// ```
    /// # Panics
    /// Panics for an empty context chunked with a positive count; malformed
    /// argument boundaries may also panic during expression parsing.
    pub fn chunks(&self, n: usize) -> Vec<String> {
        if n == 0 {
            return vec![self.context.clone()];
        }

        let chunk_size = self.context_lines.len().div_ceil(n);
        self.context_lines
            .chunks(chunk_size)
            .map(|chunk| chunk.join("\n"))
            .collect()
    }
}

fn char_index_to_byte_index(value: &str, char_index: usize) -> usize {
    if char_index == 0 {
        return 0;
    }

    value
        .char_indices()
        .nth(char_index)
        .map(|(idx, _)| idx)
        .unwrap_or(value.len())
}
