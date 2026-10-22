//! Compare returned evidence, preserving failures separately from fresh observations.
use super::{
    ProgressLedger,
    ledger::Pending,
    ledger_signature::{digest, waiting},
};
impl ProgressLedger {
    pub(super) fn result(
        &mut self,
        pending: Pending,
        content: &str,
        success: Option<bool>,
        source_turn: usize,
    ) {
        let Some((failed, output)) = super::result_output::decode(content, success) else {
            self.review.observe(source_turn, true, true);
            return;
        };
        self.observations = self.observations.saturating_add(1);
        if waiting(&pending.tool) && !failed {
            return;
        }
        let response_hash = digest(output);
        let old = self.records.iter().position(|record| {
            record.signature == pending.signature
                && record.response_hash == response_hash
                && record.failed == failed
        });
        let same = old.is_some();
        self.review.observe(source_turn, failed, same);
        if same {
            self.stagnant_results = self.stagnant_results.saturating_add(1);
        } else {
            self.novel_observations = self.novel_observations.saturating_add(1);
            self.stagnant_results = 0;
            // Changed evidence reopens blocked edits, even if the observation is read-only.
            self.epoch = self.epoch.saturating_add(1);
        }
        self.record_result(pending, response_hash, failed, source_turn, old);
    }
}
