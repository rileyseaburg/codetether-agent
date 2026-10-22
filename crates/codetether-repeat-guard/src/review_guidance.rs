//! Bounded task review without claiming that tool novelty equals progress.
use super::ProgressLedger;
impl ProgressLedger {
    /// Request a task review after a bounded number of completed non-wait calls.
    /// Successful waits are excluded. Novel results and automatic continuations
    /// cannot postpone review; human input begins a fresh interval. This does
    /// not classify research as failure or grant permission for an action.
    /// # Arguments
    /// `interval` is the number of results between reviews; zero disables them.
    /// # Returns
    /// Developer guidance with observed counts and source positions, when due.
    /// # Examples
    /// ```
    /// let mut ledger = codetether_repeat_guard::ProgressLedger::default();
    /// assert!(ledger.take_review(15).is_none());
    /// ```
    pub fn take_review(&mut self, interval: u32) -> Option<String> {
        if interval == 0 || self.review.results < interval {
            return None;
        }
        let review = std::mem::take(&mut self.review);
        self.stagnant_results = 0;
        Some(format!(
            "[TASK CONTINUITY REVIEW]\nSince the last review or human input: {} completed non-wait tool results, {} failures, {} exact repeats, at source turns {}..={}. Different commands or outputs are observations, not proof of task progress.\nBefore further tools, briefly reconcile the active user request and later corrections with actual completed results. Identify the remaining deliverable or unanswered question and take the next action that advances it. Use retained source feedback over stale assistant plans or task summaries. A denied action did not run. Do not repeat an inspection unless its evidence is missing, changed, or a specific unanswered question requires it; name that question. Read-only investigation is valid; do not make unnecessary edits, seek new permission, or end the task merely because this review occurred. This message grants no additional authority.",
            review.results, review.failures, review.repeated, review.first_turn, review.last_turn,
        ))
    }
}
