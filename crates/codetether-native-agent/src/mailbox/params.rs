//! Agent-tool parameters reconstructed for queued delivery.

use codetether_agent_input::Params;
use codetether_child_mailbox::Item;

/// Reconstruct the existing detached-message parameters for a queued item.
/// # Arguments
/// * `item` — Durable queued message with owner and prior-context ceiling.
/// # Returns
/// Message parameters retaining that owner and ceiling, with detached execution.
/// # Examples
/// Called by the application mailbox execution declaration.
pub fn queued_mailbox_params(item: &Item) -> Params {
    Params {
        action: "message".into(),
        name: None,
        instructions: None,
        message: None,
        context_id: None,
        message_images: Vec::new(),
        model: None,
        ephemeral: false,
        detach: Some(true),
        fork_turns: None,
        _current_model: None,
        parent_workspace: None,
        parent_session_id: item.parent_id.clone(),
        parent_prior_context_allowed: item.prior_context,
    }
}
