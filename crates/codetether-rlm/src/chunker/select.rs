//! Chunk selection within a token budget.

use super::types::Chunk;

/// Select chunks to fit within a token budget.
///
/// Prioritizes high-priority chunks and recent content.
pub fn select_chunks(chunks: &[Chunk], max_tokens: usize) -> Vec<Chunk> {
    let mut sorted: Vec<_> = chunks.to_vec();
    sorted.sort_by(|a, b| match b.priority.cmp(&a.priority) {
        std::cmp::Ordering::Equal => b.start_line.cmp(&a.start_line),
        other => other,
    });
    let mut selected = Vec::new();
    let mut total_tokens = 0;
    for chunk in sorted {
        if total_tokens + chunk.tokens <= max_tokens {
            selected.push(chunk.clone());
            total_tokens += chunk.tokens;
        }
    }
    selected.sort_by_key(|c| c.start_line);
    selected
}
