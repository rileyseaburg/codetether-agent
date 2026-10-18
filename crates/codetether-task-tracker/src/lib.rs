//! Bounded tracking of active protocol tasks from bus state transitions.
//!
//! ActiveTasks records nonterminal task updates and removes terminal tasks.
//! Observe bus messages as they arrive and query count for current activity.
mod state;
pub use state::ActiveTasks;
