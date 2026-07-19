//! Terminal updates for observed remote-agent turns.

use super::message::text as message;
use super::store::{TURNS, key};
use crate::provider::Role;

pub(super) fn record(name: &str, owner: Option<&str>, turn_id: &str, output: &str, failed: bool) {
    if let Some(mut turn) = TURNS.get_mut(&key(name, owner)) {
        turn.messages.push(message(Role::Assistant, output));
        if turn.turn_id == turn_id {
            turn.is_processing = false;
            turn.failed = failed;
        }
    }
}
