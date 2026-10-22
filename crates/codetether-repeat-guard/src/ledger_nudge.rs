//! Guidance for exact repeated evidence.
use super::ProgressLedger;
impl ProgressLedger {
    /// Return guidance after many identical results; never terminate investigation.
    /// # Arguments
    /// `threshold` is the number of repeated results required; zero disables nudges.
    /// # Returns
    /// Nonterminal guidance, once per threshold, or none before it is reached.
    /// # Examples
    /// ```
    /// let mut ledger = codetether_repeat_guard::ProgressLedger::default();
    /// assert!(ledger.take_nudge(15).is_none());
    /// ```
    pub fn take_nudge(&mut self, threshold: u32) -> Option<&'static str> {
        if threshold == 0 || self.stagnant_results < threshold {
            return None;
        }
        self.stagnant_results = 0;
        Some(
            "Recent tool results repeat existing evidence. Reassess the next uncertainty and change the query or approach if useful. If legitimately waiting for external work, continue with appropriate backoff. Tool activity alone does not prove task completion; do not end the task merely because this guidance appeared.",
        )
    }
}
