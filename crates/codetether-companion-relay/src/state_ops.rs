//! Create, pair, and stop operations spanning registry and runtime.
use crate::{ApiError, runtime::Runtime, state::State};
use codetether_companion_core::Error;
use codetether_companion_protocol::{PairReceipt, SessionInput, SessionReceipt};

impl State {
    /// Validate input via the core registry and attach a fresh runtime.
    pub(crate) fn create(
        &mut self,
        input: SessionInput,
        now: i64,
    ) -> Result<SessionReceipt, ApiError> {
        self.sweep(now);
        let receipt = self.registry.create(input, now)?;
        let input = self
            .registry
            .session(&receipt.id, now)?
            .input()
            .cloned()
            .ok_or(Error::Ended)?;
        self.runtimes
            .insert(receipt.id.clone(), Runtime::new(input));
        Ok(receipt)
    }
    /// Consume a pairing code and announce the paired state to viewers.
    pub(crate) fn pair(&mut self, code: &str, now: i64) -> Result<PairReceipt, ApiError> {
        let receipt = self.registry.pair(code, now)?;
        if let Some(runtime) = self.runtimes.get_mut(&receipt.id) {
            runtime.status = "paired".into();
            runtime.publish_state();
        }
        Ok(receipt)
    }
    /// Revoke the core session and stop its runtime.
    pub(crate) fn end(&mut self, id: &str, now: i64) {
        let _ = self.registry.stop(id, now);
        if let Some(runtime) = self.runtimes.get_mut(id) {
            runtime.stop();
        }
    }
    /// Stop every session (process shutdown).
    pub(crate) fn end_all(&mut self, now: i64) {
        let ids: Vec<String> = self.runtimes.keys().cloned().collect();
        for id in ids {
            self.end(&id, now);
        }
        self.sweep(now);
    }
}
