//! Normalized lexical terms for recall documents and queries.

use std::collections::HashSet;

const MAX_DOCUMENT_TOKENS: usize = 512;

/// Extract bounded unique lexical terms for a stored document.
/// # Arguments
/// * `input` - Text to normalize with the shared vector tokenizer.
/// # Returns
/// Sorted normalized terms.
/// # Examples
/// ```
/// assert_eq!(codetether_recall_index::tokens::document("beta alpha beta"), vec!["alpha", "beta"]);
/// ```
pub fn document(input: &str) -> Vec<String> {
    let mut tokens = unique(input, MAX_DOCUMENT_TOKENS, false);
    tokens.sort_unstable();
    tokens
}

/// Extract bounded unique lexical terms for a query, excluding stopwords.
/// # Arguments
/// * `input` - Text to normalize with the shared vector tokenizer.
/// # Returns
/// Encounter-ordered normalized terms.
/// # Examples
/// ```
/// assert_eq!(codetether_recall_index::tokens::query("beta alpha beta"), vec!["beta", "alpha"]);
/// ```
pub fn query(input: &str) -> Vec<String> {
    unique(input, 64, true)
}

fn unique(input: &str, limit: usize, filter_stopwords: bool) -> Vec<String> {
    let mut seen = HashSet::new();
    codetether_vectordb::tokenize::tokenize(input)
        .into_iter()
        .filter(|token| !filter_stopwords || !is_stopword(token))
        .filter(|token| seen.insert(token.clone()))
        .take(limit)
        .collect()
}

fn is_stopword(token: &str) -> bool {
    matches!(
        token,
        "a" | "an"
            | "and"
            | "are"
            | "do"
            | "for"
            | "i"
            | "in"
            | "is"
            | "it"
            | "of"
            | "on"
            | "the"
            | "this"
            | "to"
            | "we"
            | "what"
            | "with"
            | "you"
    )
}
