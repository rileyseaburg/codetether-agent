//! Lifecycle updates for observable remote-agent turns.

use super::message::text as message;
use super::store::{TURNS, key};
use super::types::{RemoteTurn, RemoteTurnGuard};
use crate::provider::Role;

/// Starts a visible remote turn and returns its cancellation-safe guard.
pub(in crate::tool::agent) fn begin(
    name: &str,
    owner: Option<&str>,
    prompt: &str,
) -> RemoteTurnGuard {
    let turn_id = uuid::Uuid::new_v4().to_string();
    let lookup = key(name, owner);
    let prompt = message(Role::User, prompt);
    if let Some(mut turn) = TURNS.get_mut(&lookup) {
        turn.turn_id.clone_from(&turn_id);
        turn.messages.push(prompt);
        turn.is_processing = true;
        turn.failed = false;
    } else {
        TURNS.insert(
            lookup,
            RemoteTurn {
                name: name.to_string(),
                owner_session_id: owner.map(ToString::to_string),
                turn_id: turn_id.clone(),
                messages: vec![prompt],
                is_processing: true,
                failed: false,
            },
        );
    }
    RemoteTurnGuard {
        name: name.to_string(),
        owner_session_id: owner.map(ToString::to_string),
        turn_id,
        settled: false,
    }
}
