//! safe char prefix for codebase indexing.

pub(in crate::indexer) fn safe_char_prefix(input: &str, max_chars: usize) -> String {
    input.chars().take(max_chars).collect()
}
