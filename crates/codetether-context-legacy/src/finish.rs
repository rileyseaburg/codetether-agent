//! Assemble provenance and residency metadata for a derived transcript.
use codetether_context_policy::{
    pages::ResidencyLevel,
    projection::helpers::{DerivedContext, messages_len_changed},
};
use codetether_provider_core::Message;
pub(super) fn finish(
    origin_len: usize,
    messages: Vec<Message>,
    before: usize,
    step0: bool,
    step1: bool,
) -> DerivedContext {
    let compressed = step0 || step1 || messages_len_changed(before, &messages);
    let mut provenance = vec!["legacy".to_string()];
    if step0 {
        provenance.push("oversized_last_message".to_string());
    }
    if step1 {
        provenance.push("context_window".to_string());
    }
    if compressed && provenance.len() == 1 {
        provenance.push("experimental".to_string());
    }
    DerivedContext {
        resolutions: vec![ResidencyLevel::Full; messages.len()],
        dropped_ranges: Vec::new(),
        provenance,
        messages,
        origin_len,
        compressed,
    }
}
