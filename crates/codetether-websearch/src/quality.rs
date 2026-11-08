//! Conservative lexical sanity check, not a semantic relevance guarantee.
use crate::{SearchFailure, result::SearchResult};
use std::collections::BTreeSet;

fn words(text: &str) -> BTreeSet<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| s.chars().count() > 1)
        .map(str::to_owned)
        .collect()
}

pub(crate) fn check(
    query: &str,
    results: Vec<SearchResult>,
) -> Result<Vec<SearchResult>, SearchFailure> {
    let mut terms = words(query);
    for filler in [
        "the", "and", "for", "in", "of", "to", "is", "an", "best", "top", "find", "search",
        "please", "today", "near", "me",
    ] {
        terms.remove(filler);
    }
    // Empty responses and queries without a lexical signal defer to the parser.
    if terms.is_empty() || results.is_empty() {
        return Ok(results);
    }
    let required = if terms.len() >= 3 { 2 } else { 1 };
    let related: Vec<_> = results
        .into_iter()
        .filter(|r| {
            let text = words(&format!("{} {} {}", r.title, r.snippet, r.url));
            terms.intersection(&text).count() >= required
        })
        .collect();
    if !related.is_empty() {
        Ok(related)
    } else {
        Err(SearchFailure::Unrelated(
            "No result had sufficient query-term overlap; try another backend. \
             Do not treat these results as evidence or invent source URLs."
                .into(),
        ))
    }
}
