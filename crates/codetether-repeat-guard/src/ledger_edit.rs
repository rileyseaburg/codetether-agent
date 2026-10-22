//! Limit identical failed edits within an evidence epoch.
use super::ProgressLedger;
impl ProgressLedger {
    /// Block only an edit that has already returned the same failure three times.
    /// Changed inputs, new observations and explicit human direction permit retries.
    /// # Arguments
    /// `tool` and `input` identify the proposed operation before runtime enrichment.
    /// # Returns
    /// A rejection only for an unchanged, repeatedly failed edit; reads and waits pass.
    /// # Examples
    /// ```
    /// let ledger = codetether_repeat_guard::ProgressLedger::default();
    /// assert!(ledger.blocked_edit("read", &serde_json::json!({})).is_none());
    /// ```
    pub fn blocked_edit(&self, tool: &str, input: &serde_json::Value) -> Option<String> {
        let (tool, input) = codetether_edit_policy::normalize_tool_call_for_execution(tool, input);
        if !super::limits::EDIT_TOOLS.contains(&tool.as_str()) {
            return None;
        }
        let signature = super::ledger_signature::signature(&tool, &input);
        self.records.iter().rev().find(|record| record.signature == signature)
            .filter(|record| record.epoch == self.epoch && record.failed && record.repeats >= 3)
            .map(|_| "REPEATED_EDIT_BLOCKED: This exact edit returned the same failure repeatedly without new evidence. Inspect the target or change the edit before retrying; a successful edit is not a failed attempt.".into())
    }
}
