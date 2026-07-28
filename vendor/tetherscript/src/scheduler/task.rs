//! Task records managed by the cooperative scheduler.

use crate::value::Value;

/// State of a cooperative task.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskState {
    Ready,
    Waiting,
    Done,
}

/// A task tracked by the scheduler.
#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub state: TaskState,
    pub result: Option<Value>,
    pub waiters: Vec<u64>,
    pub waiting_for: Vec<u64>,
}

impl Task {
    /// Create a ready task with no result or waiters.
    pub fn ready(id: u64) -> Self {
        Self {
            id,
            state: TaskState::Ready,
            result: None,
            waiters: Vec::new(),
            waiting_for: Vec::new(),
        }
    }
}
