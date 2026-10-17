//! tokenize for embedding for codebase indexing.

pub(in crate::indexer) fn tokenize_for_embedding(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            current.push(ch.to_ascii_lowercase());
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
            if tokens.len() >= 4096 {
                return tokens;
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
