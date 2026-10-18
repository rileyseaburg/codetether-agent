//! Snapshot scheduling for proactive session-context work.

use super::{registry, types};

/// Schedule the newest generation if the transcript is long enough and enabled.
/// # Arguments
/// * `session_id` — Registered native session ID.
/// * `message_count` — Current transcript length used by the minimum-size gate.
/// * `spawn_worker` — Native worker launcher called only when no worker is active.
pub fn schedule(session_id: &str, message_count: usize, spawn_worker: fn(String)) {
    if message_count < 12 || disabled() {
        return;
    }
    let spawn = registry::enqueue(session_id, |runtime, generation| types::Snapshot {
        session_id: session_id.to_string(),
        messages: Vec::new(),
        index: codetether_context_policy::summary_index::SummaryIndex::new(),
        runtime,
        generation,
    });
    if spawn {
        spawn_worker(session_id.to_string());
    }
}

fn disabled() -> bool {
    std::env::var("CODETETHER_RLM_PROACTIVE")
        .is_ok_and(|value| matches!(value.trim(), "0" | "off" | "false"))
}
