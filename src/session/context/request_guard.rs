//! Final request budgeting after RLM resolution and dynamic context assembly.

use crate::provider::{CompletionRequest, Message, Role};
use crate::session::helper::token::{
    estimate_request_tokens, estimate_tokens_for_messages, session_completion_max_tokens,
};
use anyhow::{Result, ensure};

/// Reject oversized mandatory context; include optional context only if it fits.
///
/// Optional recall/LSP messages are inserted before the active user turn, never
/// ahead of the stable history prefix or inside a tool-call/result exchange.
/// These are evidence, not policy: system/developer extras become user context
/// so providers cannot lift changing recall into the cache's system prefix.
/// Returns a request or a context-window error understood by overflow recovery.
pub(crate) fn finish(
    mut request: CompletionRequest,
    extras: Vec<Message>,
) -> Result<CompletionRequest> {
    let budget = super::input_budget::for_completion(
        &request.model,
        request
            .max_tokens
            .unwrap_or_else(session_completion_max_tokens),
    );
    let mut used = estimate_request_tokens("", &request.messages, &request.tools);
    ensure!(
        used <= budget,
        "context window budget exceeded: estimated input {used} tokens exceeds {budget} after output reservation"
    );
    let mut insertion = super::active_tail::active_user_tail_start(&request.messages, 0)
        .unwrap_or(request.messages.len());
    for mut message in extras {
        if matches!(message.role, Role::System | Role::Developer) {
            message.role = Role::User;
        }
        let cost = estimate_tokens_for_messages(std::slice::from_ref(&message));
        if cost > budget.saturating_sub(used) {
            tracing::debug!(
                cost,
                used,
                budget,
                "Omitting optional context that exceeds input budget"
            );
            continue;
        }
        request.messages.insert(insertion, message);
        insertion += 1;
        used += cost;
    }
    Ok(request)
}

#[cfg(test)]
#[path = "request_guard_tests.rs"]
mod tests;
