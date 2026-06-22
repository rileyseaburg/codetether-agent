//! State for the `/edit` fuzzy file finder overlay.
//!
//! Holds the workspace-relative candidate paths (scanned once on open), the
//! current query, and the ranked match list. Pure data + ranking; no terminal
//! or filesystem side effects beyond the initial scan performed by the caller.

use std::path::PathBuf;

use crate::tui::app::state::session_fuzzy::fuzzy_score;

/// One ranked candidate: its workspace-relative display string and full path.
#[derive(Debug, Clone)]
pub struct FuzzyEntry {
    pub display: String,
    pub path: PathBuf,
}

/// Active fuzzy-finder overlay state.
#[derive(Debug, Clone, Default)]
pub struct FuzzyFindState {
    pub query: String,
    pub candidates: Vec<FuzzyEntry>,
    pub results: Vec<FuzzyEntry>,
    pub selected: usize,
}

impl FuzzyFindState {
    /// Builds a finder over `candidates`, showing all of them initially.
    pub fn new(candidates: Vec<FuzzyEntry>) -> Self {
        let results = candidates.clone();
        Self {
            query: String::new(),
            candidates,
            results,
            selected: 0,
        }
    }

    /// Re-ranks candidates against the current query.
    pub fn rerank(&mut self) {
        rerank_into(&self.query, &self.candidates, &mut self.results);
        self.selected = 0;
    }

    /// Currently highlighted entry, if any.
    pub fn current(&self) -> Option<&FuzzyEntry> {
        self.results.get(self.selected)
    }
}

/// Ranks `candidates` against `query`, writing the sorted matches into `out`.
pub fn rerank_into(query: &str, candidates: &[FuzzyEntry], out: &mut Vec<FuzzyEntry>) {
    if query.is_empty() {
        *out = candidates.to_vec();
        return;
    }
    let mut scored: Vec<(u32, &FuzzyEntry)> = candidates
        .iter()
        .filter_map(|c| fuzzy_score(query, &c.display).map(|s| (s, c)))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.display.cmp(&b.1.display)));
    *out = scored.into_iter().map(|(_, c)| c.clone()).collect();
}
