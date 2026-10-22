//! Mandatory input budgets and optional-context insertion.
use crate::derivation::completion_limit::session_completion_max_tokens;
use anyhow::{Result, ensure};
use codetether_provider_core::{CompletionRequest, Message, Role, ToolDefinition};

/// Reject oversized mandatory context; include optional context only if it fits.
///
/// Optional recall/LSP messages precede a fresh user input, or follow completed
/// history during a continuing tool run. They never split a call/result exchange.
/// These are evidence, not policy: system/developer extras become user context
/// so providers cannot lift changing recall into the cache's system prefix.
/// Returns a request or a context-window error understood by overflow recovery.
///
/// # Arguments
/// * `request` - Mandatory context and requested output limit.
/// * `extras` - Optional evidence considered in the supplied order.
/// * `estimate_request` - Estimate of mandatory messages and tools.
/// * `estimate_messages` - Estimate of an optional message slice.
/// * `on_omit` - Observer for an oversized optional message.
///
/// # Returns
/// A request containing only optional messages that fit the remaining budget.
///
/// # Errors
/// Returns a context-window error when mandatory input exceeds the safe budget.
///
/// # Examples
/// See [super::from_derived] for constructing the request.
pub fn finish(
    mut request: CompletionRequest,
    extras: Vec<Message>,
    estimate_request: impl Fn(&[Message], &[ToolDefinition]) -> usize,
    estimate_messages: impl Fn(&[Message]) -> usize,
    mut on_omit: impl FnMut(usize, usize, usize),
) -> Result<CompletionRequest> {
    let budget = crate::derivation::input_budget::for_completion(
        &request.model,
        request
            .max_tokens
            .unwrap_or_else(session_completion_max_tokens),
    );
    let mut used = estimate_request(&request.messages, &request.tools);
    ensure!(
        used <= budget,
        "context window budget exceeded: estimated input {used} tokens exceeds {budget} after output reservation"
    );
    let mut insertion = super::messages::evidence_boundary(&request.messages);
    for mut message in extras {
        if matches!(message.role, Role::System | Role::Developer) {
            message.role = Role::User;
        }
        let cost = estimate_messages(std::slice::from_ref(&message));
        if cost > budget.saturating_sub(used) {
            on_omit(cost, used, budget);
            continue;
        }
        request.messages.insert(insertion, message);
        insertion += 1;
        used += cost;
    }
    Ok(request)
}
