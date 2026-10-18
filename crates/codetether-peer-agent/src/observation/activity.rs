//! Structured in-flight activity copied from A2A task metadata.

use super::store::{TURNS, key};
use serde_json::Value;

pub fn record(name: &str, owner: Option<&str>, events: &[Value]) {
    let Some(mut turn) = TURNS.get_mut(&key(name, owner)) else {
        return;
    };
    turn.activity = events.iter().rev().take(100).rev().cloned().collect();
}
