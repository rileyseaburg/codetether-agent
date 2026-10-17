//! build embedding input for codebase indexing.

use super::*;

pub(in crate::indexer) fn build_embedding_input(
    path: &str,
    language: &str,
    content: &str,
    max_chars: usize,
) -> String {
    let snippet = safe_char_prefix(content, max_chars);
    format!("path:{path}\nlanguage:{language}\n\n{snippet}")
}
