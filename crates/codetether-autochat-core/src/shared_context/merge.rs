//! Shared relay memory merge operations.
use super::*;
impl SharedRelayContext {
    pub fn merge_delta(&mut self, delta: &ContextDelta) {
        merge_bucket(&mut self.facts, &delta.facts);
        merge_bucket(&mut self.decisions, &delta.decisions);
        merge_bucket(&mut self.risks, &delta.risks);
        merge_bucket(&mut self.next_actions, &delta.next_actions);
        merge_bucket(&mut self.evidence, &delta.evidence);
    }
}
