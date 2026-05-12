//! Top-level compress function combining chunk + select + reassemble.

use super::chunk::chunk;
use super::reassemble::reassemble;
use super::select::select_chunks;
use super::types::ChunkOptions;

/// Intelligently compress content to fit within token budget.
pub fn compress(content: &str, max_tokens: usize, options: Option<ChunkOptions>) -> String {
    let chunks = chunk(content, options);
    let selected = select_chunks(&chunks, max_tokens);
    reassemble(&selected)
}