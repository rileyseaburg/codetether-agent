//! Stable fuzzy ranking by session identifier or title.
use codetether_session_metadata::SessionSummary;
/// Rank matching sessions while retaining original indices and tie order.
///
/// # Arguments
/// * `sessions` - Session listing in its original order.
/// * `filter` - Fuzzy identifier or title query.
///
/// # Returns
/// Borrowed matches and their original indices; an empty query keeps all rows.
///
/// # Examples
/// ```
/// assert!(codetether_session_picker::filtered_sessions(&[], "query").is_empty());
/// ```

pub fn filtered_sessions<'a>(
    sessions: &'a [SessionSummary],
    filter: &str,
) -> Vec<(usize, &'a SessionSummary)> {
    if filter.is_empty() {
        return sessions.iter().enumerate().collect();
    }
    let needle = filter;
    let mut scored: Vec<(u32, usize, &SessionSummary)> = sessions
        .iter()
        .enumerate()
        .filter_map(|(i, s)| best_score(needle, s).map(|sc| (sc, i, s)))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    scored.into_iter().map(|(_, i, s)| (i, s)).collect()
}

/// Best fuzzy score for `needle` across a session's id and title.
fn best_score(needle: &str, s: &SessionSummary) -> Option<u32> {
    let id = codetether_util::fuzzy::fuzzy_score(needle, &s.id);
    let title = s
        .title
        .as_deref()
        .and_then(|t| codetether_util::fuzzy::fuzzy_score(needle, t));
    id.into_iter().chain(title).max()
}
