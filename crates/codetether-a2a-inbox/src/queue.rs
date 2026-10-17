//! Process-wide queue of inbound turns awaiting the interactive session.

use std::collections::VecDeque;
use std::sync::{LazyLock, Mutex};

use super::pending::{Pending, Responder, channel};

/// A peer's turn, parked until the TUI is idle enough to run it.
///
/// # Examples
/// ```rust
/// let _pending = codetether_a2a_inbox::enqueue("task", Some("context"), "peer", "hello");
/// let inbound = codetether_a2a_inbox::dequeue().unwrap();
/// assert_eq!(inbound.task_id, "task");
/// ```
pub struct Inbound {
    /// Remote task identifier.
    pub task_id: String,
    /// Optional conversation context.
    pub context_id: Option<String>,
    /// Name of the sending peer.
    pub from: String,
    /// Prompt accepted from the peer.
    pub prompt: String,
    /// One-shot completion handle for this turn.
    pub responder: Responder,
}

static QUEUE: LazyLock<Mutex<VecDeque<Inbound>>> = LazyLock::new(|| Mutex::new(VecDeque::new()));

/// Park a turn for the live session and return the slot the server awaits.
///
/// # Arguments
/// * `task_id` - Remote task identifier.
/// * `context_id` - Optional conversation context.
/// * `from` - Sending peer name.
/// * `prompt` - Prompt to hand to the live session.
/// # Returns
/// A future resolving when the receiving session responds or drops the turn.
/// # Panics
/// Panics if the inbox mutex was poisoned.
/// # Examples
/// ```rust
/// let _pending = codetether_a2a_inbox::enqueue("task", None, "peer", "hello");
/// assert!(codetether_a2a_inbox::dequeue().is_some());
/// ```
pub fn enqueue(task_id: &str, context_id: Option<&str>, from: &str, prompt: &str) -> Pending {
    let (pending, responder) = channel();
    QUEUE.lock().expect("live inbox lock").push_back(Inbound {
        task_id: task_id.to_string(),
        context_id: context_id.map(ToString::to_string),
        from: from.to_string(),
        prompt: prompt.to_string(),
        responder,
    });
    tracing::info!(target: "codetether_agent::a2a::live_inbox::queue",
        task_id,
        from,
        "Parked inbound A2A turn for the interactive session"
    );
    pending
}

/// Take the oldest parked turn, if any.
///
/// # Arguments
/// Takes no arguments.
/// # Returns
/// The oldest waiting turn, or None when the queue is empty.
/// # Panics
/// Panics if the inbox mutex was poisoned.
/// # Examples
/// ```rust
/// let _pending = codetether_a2a_inbox::enqueue("task", None, "peer", "hello");
/// assert_eq!(codetether_a2a_inbox::dequeue().unwrap().from, "peer");
/// ```
pub fn dequeue() -> Option<Inbound> {
    QUEUE.lock().expect("live inbox lock").pop_front()
}
