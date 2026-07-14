//! Snapshot scheduling for proactive session-context work.

use crate::session::Session;

use super::{registry, types, worker};

pub(super) fn run(session: &Session) {
    if session.messages.len() < 12 || disabled() {
        return;
    }
    let spawn = registry::enqueue(&session.id, |runtime, generation| types::Snapshot {
        session_id: session.id.clone(),
        messages: session.messages.clone(),
        index: crate::session::index::SummaryIndex::new(),
        runtime,
        generation,
    });
    if spawn {
        worker::spawn(session.id.clone());
    }
}

fn disabled() -> bool {
    std::env::var("CODETETHER_RLM_PROACTIVE")
        .is_ok_and(|value| matches!(value.trim(), "0" | "off" | "false"))
}
