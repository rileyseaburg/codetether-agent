//! Direct-swarm lifecycle observation and delivery to the shared swarm view.
//!
//! Create an [Observer] for an invocation, send worker events using its sender,
//! then [drain] into a [codetether_swarm_view::SwarmViewState] on frontend ticks.
//! Dropping an unfinished observer emits a cancellation error.

#[path = "cancel.rs"]
mod cancel;
#[path = "channel.rs"]
mod channel;
#[path = "finish.rs"]
mod finish;
#[path = "observer.rs"]
mod observer;
#[path = "stats.rs"]
mod stats;
#[path = "task.rs"]
mod task;

pub use observer::Observer;
pub use task::started;

/// Resolve the stable identifier used by worker and monitor records.
/// # Arguments
/// * `task` — Input task with an optional explicit identifier.
/// * `index` — Zero-based task position.
/// # Returns
/// Explicit identifier or a one-based task label.
/// # Examples
/// ```
/// let task = codetether_swarm_tool::task_input::TaskInput { id: None, name: "Review".into(), instruction: "Inspect".into(), specialty: None, needs_worktree: None };
/// assert_eq!(codetether_swarm_monitor::task_id(&task, 0), "task-1");
/// ```
pub fn task_id(task: &codetether_swarm_tool::task_input::TaskInput, index: usize) -> String {
    task::id(task, index)
}

/// Apply queued direct-swarm events and the view's own pending events.
/// # Arguments
/// * `view` — Frontend swarm state to update.
/// # Returns
/// Whether either event source changed the view. Both sources are always drained.
/// # Examples
/// ```
/// let mut view = codetether_swarm_view::SwarmViewState::default();
/// let _changed = codetether_swarm_monitor::drain(&mut view);
/// ```
pub fn drain(view: &mut codetether_swarm_view::SwarmViewState) -> bool {
    channel::drain(view) | view.drain_events()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

mod runtime;

mod start;
