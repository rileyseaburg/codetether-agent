//! Emit a bounded chunk or split an oversized one.

use crate::chunker::{
    estimate::estimate_tokens,
    split::split_large_chunk,
    types::{Chunk, ChunkOptions, ChunkType},
};

pub(super) fn flush_chunk(
    out: &mut Vec<Chunk>,
    lines: &[&str],
    start: usize,
    end: usize,
    ctype: ChunkType,
    opts: &ChunkOptions,
) {
    let content = lines.join("\n");
    let tokens = estimate_tokens(&content);
    if tokens > opts.max_chunk_tokens {
        out.extend(split_large_chunk(
            lines,
            start,
            ctype,
            opts.max_chunk_tokens,
        ));
    } else {
        out.push(Chunk {
            content,
            chunk_type: ctype,
            start_line: start,
            end_line: end.saturating_sub(1),
            tokens,
            priority: 3,
        });
    }
}
