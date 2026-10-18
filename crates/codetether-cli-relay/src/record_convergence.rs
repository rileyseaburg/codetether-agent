//! Track consecutive equivalent specialist outputs.
use crate::{Host, convergence::normalize_for_convergence, state::Runtime};
impl<H: Host> Runtime<H> {
    pub(super) fn record_convergence(&mut self, output: &str) {
        let normalized = normalize_for_convergence(&output);
        if self.previous_normalized.as_deref() == Some(normalized.as_str()) {
            self.convergence_hits += 1;
        } else {
            self.convergence_hits = 0;
        }
        self.previous_normalized = Some(normalized);
    }
}
