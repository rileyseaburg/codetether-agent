//! Process-wide queue of inbound turns awaiting the interactive session.

use std::collections::VecDeque;
use std::sync::{LazyLock, Mutex};

use super::pending::{Pending, Responder, channel};

/// A peer's turn, parked until the TUI is idle enough to run it.
pub struct Inbound {
    pub task_id: String,
    pub context_id: Option<String>,
    pub from: String,
    pub prompt: String,
    pub responder: Responder,
}

static QUEUE: LazyLock<Mutex<VecDeque<Inbound>>> = LazyLock::new(|| Mutex::new(VecDeque::new()));

/// Park a turn for the live session and return the slot the server awaits.
pub fn enqueue(task_id: &str, context_id: Option<&str>, from: &str, prompt: &str) -> Pending {
    let (pending, responder) = channel();
    QUEUE.lock().expect("live inbox lock").push_back(Inbound {
        task_id: task_id.to_string(),
        context_id: context_id.map(ToString::to_string),
        from: from.to_string(),
        prompt: prompt.to_string(),
        responder,
    });
    tracing::info!(
        task_id,
        from,
        "Parked inbound A2A turn for the interactive session"
    );
    pending
}

/// Take the oldest parked turn, if any.
pub fn dequeue() -> Option<Inbound> {
    QUEUE.lock().expect("live inbox lock").pop_front()
}
