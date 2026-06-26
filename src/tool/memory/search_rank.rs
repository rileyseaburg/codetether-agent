//! Candidate collection and filtering for memory [`search`](super::search).

use super::{MemoryEntry, query_match};

/// A search candidate: the entry plus its lexical token-overlap score.
pub(super) struct Candidate {
    pub entry: MemoryEntry,
    pub lexical: usize,
}

/// Whether `entry` passes the optional tag and scope filters.
pub(super) fn matches_filters(
    entry: &MemoryEntry,
    tags: Option<&[String]>,
    scope: Option<&str>,
) -> bool {
    if let Some(search_tags) = tags
        && !search_tags.is_empty()
        && !search_tags.iter().any(|t| entry.tags.contains(t))
    {
        return false;
    }
    if let Some(scope) = scope
        && entry.scope.as_deref() != Some(scope)
    {
        return false;
    }
    true
}

/// Build a candidate from an entry, dropping it when no query token matches.
pub(super) fn make_candidate(entry: &mut MemoryEntry, query: Option<&str>) -> Option<Candidate> {
    let lexical = match query {
        Some(q) => query_match::query_score(entry, q),
        None => 1,
    };
    if lexical == 0 {
        return None;
    }
    entry.touch();
    Some(Candidate {
        entry: entry.clone(),
        lexical,
    })
}
