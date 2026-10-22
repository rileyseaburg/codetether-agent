//! Trusted human input starts a new review interval.
use super::ProgressLedger;
impl ProgressLedger {
    /// Reset retry restrictions for human input while retaining runtime continuations.
    /// # Arguments
    /// `message` is supplied by the application's trusted-human ingress path.
    /// # Returns
    /// Unit; known automatic continuation prompts do not reset the ledger.
    /// # Examples
    /// ```
    /// use codetether_provider_core::{Message, Role};
    /// let mut ledger = codetether_repeat_guard::ProgressLedger::default();
    /// ledger.human_input(&Message { role: Role::User, content: vec![] });
    /// assert_eq!(ledger.stagnant_results, 0);
    /// ```
    pub fn human_input(&mut self, message: &codetether_provider_core::Message) {
        let automatic = message.content.iter().any(|part| {
            if let codetether_provider_core::ContentPart::Text { text } = part {
                text.starts_with(
                    "Continue this live CodeTether run from its structured checkpoint.",
                ) || (text.starts_with("Continue working toward the active thread goal.")
                    && text.contains("<objective>"))
            } else {
                false
            }
        });
        if !automatic {
            self.new_user_epoch();
        }
    }
    /// Begin a new human-directed investigation without discarding audit counters.
    /// # Examples
    /// ```
    /// let mut ledger = codetether_repeat_guard::ProgressLedger::default();
    /// ledger.stagnant_results = 10;
    /// ledger.new_user_epoch();
    /// assert_eq!(ledger.stagnant_results, 0);
    /// ```
    pub fn new_user_epoch(&mut self) {
        self.epoch = self.epoch.saturating_add(1);
        self.stagnant_results = 0;
        self.review = Default::default();
    }
}
