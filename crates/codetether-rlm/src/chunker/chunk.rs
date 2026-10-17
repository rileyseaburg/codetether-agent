//! Accumulate content between semantic boundaries.

use super::{
    boundaries::find_boundaries,
    types::{Chunk, ChunkOptions, ChunkType},
};
mod flush;
use flush::flush_chunk;

/// Split content into semantic chunks.
///
/// # Arguments
/// `content` is the source text; `options` controls splitting limits.
///
/// # Returns
/// Ordered chunks with original line spans.
///
/// # Examples
/// ```
/// use codetether_rlm::RlmChunker;
/// let chunks = RlmChunker::chunk("hello", None);
/// assert_eq!(chunks[0].content, "hello");
/// ```
pub fn chunk(content: &str, options: Option<ChunkOptions>) -> Vec<Chunk> {
    let opts = options.unwrap_or_default();
    let lines: Vec<&str> = content.lines().collect();
    let boundaries = find_boundaries(&lines);
    let mut chunks = Vec::new();
    let (mut cur, mut ctype, mut start, mut pri) = (Vec::<&str>::new(), ChunkType::Text, 0, 1u8);
    for (i, line) in lines.iter().enumerate() {
        if let Some((bt, bp)) = boundaries.get(&i)
            && !cur.is_empty()
        {
            flush_chunk(&mut chunks, &cur, start, i, ctype, &opts);
            cur = Vec::new();
            start = i;
            ctype = *bt;
            pri = *bp;
        }
        cur.push(line);
        if i >= lines.len().saturating_sub(opts.preserve_recent) {
            pri = pri.max(8);
        }
    }
    if !cur.is_empty() {
        flush_chunk(&mut chunks, &cur, start, lines.len(), ctype, &opts);
    }
    chunks
}
