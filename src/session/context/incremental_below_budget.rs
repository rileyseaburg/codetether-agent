//! Early-return for `derive_incremental` when the full history already
//! fits the token budget.

use crate::provider::{Message, ToolDefinition};
use crate::session::ResidencyLevel;
use crate::session::helper::experimental;
use crate::session::helper::token::estimate_request_tokens;

use super::helpers::DerivedContext;

/// Return `Some(DerivedContext)` when `clone` already fits
/// `budget_tokens`. The pairing-repair pass still runs so the LLM
/// never sees orphaned tool calls.
pub(super) fn try_pass_through(
    clone: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
    origin_len: usize,
) -> Option<DerivedContext> {
    if estimate_request_tokens(system_prompt, clone, tools) > budget_tokens {
        return None;
    }
    experimental::pairing::repair_orphans(clone);
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
