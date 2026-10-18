//! Finder initialization, ranking updates and selected entry lookup.
use crate::{FuzzyEntry, FuzzyFindState, rerank_into};

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
