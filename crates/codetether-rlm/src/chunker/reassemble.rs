//! Chunk reassembly into a single string.

use super::types::Chunk;

/// Reassemble selected chunks into a single string.
pub fn reassemble(chunks: &[Chunk]) -> String {
    if chunks.is_empty() { return String::new(); }
    let mut parts = Vec::new();
    let mut last_end: Option<usize> = None;
    for chunk in chunks {
        if let Some(end) = last_end && chunk.start_line > end + 1 {
            let gap = chunk.start_line - end - 1;
            parts.push(format!("\n[... {} lines omitted ...]\n", gap));
        }
        parts.push(chunk.content.clone());
        last_end = Some(chunk.end_line);
    }
    parts.join("\n")
}