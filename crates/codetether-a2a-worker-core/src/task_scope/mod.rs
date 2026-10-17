//! Scope gating for incoming A2A tasks and bus events.
//!
//! Agents must only act on work that falls within their declared scope.
//! This module centralises the "is this task/event for me?" check so
//! every intake path (SSE, polling, bus broadcast) applies the same rules.
//!
//! # Scope dimensions
//!
//! | Dimension       | Where it comes from                    | Reject if                         |
//! |-----------------|----------------------------------------|-----------------------------------|
//! | `target_agent`  | task metadata `target_agent_name`      | non-empty AND ≠ worker agent name |
//! | `target_worker` | task metadata `target_worker_id`        | non-empty AND ≠ worker ID         |
//! | `workspace`     | task `workspace_id` / metadata         | not in worker's registered set*   |
//!
//! \* An empty/absent workspace ID is treated as a global broadcast
//!   (virtual task) and is **always** accepted.

//!
//! Use [`check_task_scope`] before dispatching a task. The policy checks agent,
//! worker, then workspace targeting and returns the first rejection reason.
mod check;
mod fields;
pub use check::check_task_scope;
#[cfg(test)]
mod tests_1;
#[cfg(test)]
mod tests_2;
#[cfg(test)]
mod tests_3;
