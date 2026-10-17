//! extract quoted literal for codebase indexing.

pub(in crate::indexer) fn extract_quoted_literal(line: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let mut parts = line.split(quote);
        let _ = parts.next();
        if let Some(value) = parts.next()
            && !value.trim().is_empty()
        {
            return Some(value.trim().to_string());
        }
    }
    None
}
