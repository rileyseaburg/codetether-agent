//! approximate token count for codebase indexing.

pub(in crate::indexer) fn approximate_token_count(text: &str) -> u64 {
    let words = text.split_whitespace().count() as u64;
    words.max(1)
}
