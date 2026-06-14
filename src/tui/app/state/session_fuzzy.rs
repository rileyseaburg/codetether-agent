//! Fuzzy subsequence scoring for session picker filtering.
//!
//! Case-insensitive subsequence matcher: every character of `needle`
//! must appear in `haystack` in order (not necessarily adjacent).
//! Consecutive matches score higher so tighter matches rank first.

/// Score `needle` against `haystack`.
///
/// Returns `None` when `needle` is not a subsequence of `haystack`.
/// Higher scores indicate tighter (more consecutive) matches.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::app::state::session_fuzzy::fuzzy_score;
///
/// assert!(fuzzy_score("a3f", "a3f9-1c2d").is_some());
/// assert!(fuzzy_score("af9", "a3f9-1c2d").is_some()); // scattered
/// assert!(fuzzy_score("zzz", "a3f9-1c2d").is_none());
/// // Contiguous match outscores scattered match
/// let tight = fuzzy_score("a3f", "a3f9").unwrap();
/// let loose = fuzzy_score("a3f", "a-3-f").unwrap();
/// assert!(tight > loose);
/// ```
pub fn fuzzy_score(needle: &str, haystack: &str) -> Option<u32> {
    if needle.is_empty() {
        return Some(0);
    }
    let needle = needle.to_lowercase();
    let haystack = haystack.to_lowercase();
    let mut hay = haystack.char_indices();
    let mut score = 0u32;
    let mut run = 0u32;
    let mut last_end: Option<usize> = None;

    for nc in needle.chars() {
        let (idx, hc) = hay.by_ref().find(|(_, hc)| *hc == nc)?;
        if last_end == Some(idx) {
            run += 1;
            score += 2 + run;
        } else {
            run = 0;
            score += 1;
        }
        last_end = Some(idx + hc.len_utf8());
    }
    Some(score)
}
