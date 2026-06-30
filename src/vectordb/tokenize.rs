//! Tokenizer shared by the local embedding engine.

/// Split `input` into lowercase alphanumeric/underscore tokens.
///
/// Caps output at 4096 tokens to bound work on large inputs.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::vectordb::tokenize::tokenize;
///
/// assert_eq!(tokenize("Hello, world!"), vec!["hello", "world"]);
/// ```
pub fn tokenize(input: &str) -> Vec<String> {
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
