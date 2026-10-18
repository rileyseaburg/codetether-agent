//! Query, candidate catalog and ranked selection state.
use crate::FuzzyEntry;

/// Active fuzzy-finder overlay state.
/// # Examples
/// ```
/// let state = codetether_fuzzy_finder::FuzzyFindState::new(vec![]);
/// assert!(state.current().is_none());
/// ```
#[derive(Debug, Clone, Default)]
pub struct FuzzyFindState {
    /// Current subsequence query.
    pub query: String,
    /// Original candidate catalog.
    pub candidates: Vec<FuzzyEntry>,
    /// Ranked entries matching the query.
    pub results: Vec<FuzzyEntry>,
    /// Index of the highlighted ranked entry.
    pub selected: usize,
}
