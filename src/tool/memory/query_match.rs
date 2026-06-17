//! Query matching for the memory search action.
//!
//! The legacy matcher required the *entire* query string to appear as a
//! contiguous substring, so a query like `"working project directory"` failed
//! to match an entry containing `"Riley's working project is /home/..."`. This
//! module tokenizes the query and matches on individual words, returning a
//! relevance score (number of distinct query tokens found in the content or
//! tags) so callers can rank by relevance.

use super::MemoryEntry;

/// Split a query into lowercase word tokens (alphanumeric runs).
fn tokenize(query: &str) -> Vec<String> {
    query
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
        .collect()
}

/// Relevance score for `entry` against `query`.
///
/// Returns the count of distinct query tokens that appear in the entry's
/// content or any tag. `0` means no match. An empty/whitespace query scores
/// `1` so it acts as a match-all filter (callers still rank by importance).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::memory::query_match::query_score;
/// use codetether_agent::tool::memory::MemoryEntry;
///
/// let entry = MemoryEntry::new("Riley's working project is /home/foo", vec![]);
/// assert!(query_score(&entry, "working project directory") > 0);
/// assert_eq!(query_score(&entry, "kubernetes deployment"), 0);
/// ```
pub fn query_score(entry: &MemoryEntry, query: &str) -> usize {
    let tokens = tokenize(query);
    if tokens.is_empty() {
        return 1;
    }
    let content = entry.content.to_lowercase();
    let tags: Vec<String> = entry.tags.iter().map(|t| t.to_lowercase()).collect();
    tokens
        .iter()
        .filter(|tok| content.contains(*tok) || tags.iter().any(|t| t.contains(*tok)))
        .count()
}
