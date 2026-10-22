//! Retain bounded call/result identities and per-epoch failure counts.
use super::{
    ProgressLedger,
    ledger::{Pending, Record},
};
impl ProgressLedger {
    pub(super) fn record_result(
        &mut self,
        pending: Pending,
        response_hash: String,
        failed: bool,
        source_turn: usize,
        old: Option<usize>,
    ) {
        let repeats = if old.is_some() && old.is_some_and(|i| self.records[i].epoch == self.epoch) {
            self.records[old.unwrap()].repeats.saturating_add(1)
        } else {
            1
        };
        if let Some(index) = old {
            self.records.remove(index);
        }
        self.records.push(Record {
            signature: pending.signature,
            response_hash,
            failed,
            repeats,
            epoch: self.epoch,
            source_turn,
        });
        if self.records.len() > 128 {
            self.records.remove(0);
        }
    }
}
