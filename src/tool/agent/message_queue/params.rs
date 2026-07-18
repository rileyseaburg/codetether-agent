//! Agent-tool parameters reconstructed for queued delivery.

use super::item::Item;
use crate::tool::agent::params::Params;

pub(super) fn queued(item: &Item) -> Params {
    Params {
        action: "message".into(),
        name: None,
        instructions: None,
        message: None,
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
