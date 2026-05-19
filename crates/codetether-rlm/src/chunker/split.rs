//! Split oversized chunks into smaller pieces.

use super::estimate::estimate_tokens;
use super::types::{Chunk, ChunkType};

/// Split a large chunk into smaller pieces.
pub fn split_large_chunk(
    lines: &[&str],
    start_line: usize,
    chunk_type: ChunkType,
    max_tokens: usize,
) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    let mut current_tokens = 0;
    let mut current_start = start_line;
    for (i, line) in lines.iter().enumerate() {
        let lt = estimate_tokens(line);
        if current_tokens + lt > max_tokens && !current.is_empty() {
            chunks.push(Chunk {
                content: current.join("\n"),
                chunk_type,
                start_line: current_start,
                end_line: start_line + i - 1,
                tokens: current_tokens,
                priority: 3,
            });
            current = Vec::new();
            current_tokens = 0;
            current_start = start_line + i;
        }
        current.push(line);
        current_tokens += lt;
    }
    if !current.is_empty() {
        chunks.push(Chunk {
            content: current.join("\n"),
            chunk_type,
            start_line: current_start,
            end_line: start_line + lines.len() - 1,
            tokens: current_tokens,
            priority: 3,
        });
    }
    chunks
}
