//! Early-return for `derive_incremental` when the full history already
//! fits the token budget.

use crate::pages::ResidencyLevel;
use crate::transforms::pairing;
use codetether_provider_core::Message;

use super::helpers::DerivedContext;

/// Return `Some(DerivedContext)` when `clone` already fits
/// `budget_tokens`. The pairing-repair pass still runs so the LLM
/// never sees orphaned tool calls.
pub fn try_pass_through(
    clone: &mut Vec<Message>,
    budget_tokens: usize,
    origin_len: usize,
    estimate_request: impl Fn(&[Message]) -> usize,
) -> Option<DerivedContext> {
    if estimate_request(clone) > budget_tokens {
        return None;
    }
    pairing::repair_orphans(clone);
    let messages = std::mem::take(clone);
    Some(DerivedContext {
        resolutions: vec![ResidencyLevel::Full; messages.len()],
        dropped_ranges: Vec::new(),
        provenance: vec!["incremental_below_budget".to_string()],
        messages,
        origin_len,
        compressed: false,
    })
}
