//! Main chunking logic — splits content into semantic chunks.

use super::boundaries::find_boundaries;
use super::estimate::estimate_tokens;
use super::split::split_large_chunk;
use super::types::{Chunk, ChunkOptions, ChunkType};

/// Split content into semantic chunks.
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

fn flush_chunk(
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
        out.extend(split_large_chunk(lines, start, ctype, opts.max_chunk_tokens));
    } else {
        out.push(Chunk { content, chunk_type: ctype, start_line: start, end_line: end.saturating_sub(1), tokens, priority: 3 });
    }
}
