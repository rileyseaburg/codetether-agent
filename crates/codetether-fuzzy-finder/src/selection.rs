//! Wrapped navigation through ranked finder results.
//! FuzzyFindState keeps selection unchanged when there are no results.
use crate::FuzzyFindState;
impl FuzzyFindState {
    /// Move selection by delta, wrapping at both ends.
    ///
    /// # Arguments
    /// * `delta` — Signed row offset.
    ///
    /// # Returns
    /// Updates selection only when results exist.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_fuzzy_finder::FuzzyFindState::default();
    /// state.move_selection(-1);
    /// assert_eq!(state.selected, 0);
    /// ```

    pub fn move_selection(&mut self, delta: isize) {
        let len = self.results.len();
        if len == 0 {
            return;
        }
        let next = (self.selected as isize + delta).rem_euclid(len as isize);
        self.selected = next as usize;
    }
}
