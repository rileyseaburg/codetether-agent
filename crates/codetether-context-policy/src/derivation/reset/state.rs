//! Owned reset state shared by the preparation stages.
use super::*;
pub(super) struct ResetState {
    pub messages: Vec<Message>,
    pub dropped_ranges: Vec<(usize, usize)>,
    pub provenance: Vec<String>,
    pub base_index: usize,
    pub origin_len: usize,
}
impl ResetState {
    pub(super) fn complete(mut self, compressed: bool) -> DerivedContext {
        pairing::repair_orphans(&mut self.messages);
        DerivedContext {
            resolutions: vec![ResidencyLevel::Full; self.messages.len()],
            dropped_ranges: self.dropped_ranges,
            provenance: self.provenance,
            messages: self.messages,
            origin_len: self.origin_len,
            compressed,
        }
    }
}
