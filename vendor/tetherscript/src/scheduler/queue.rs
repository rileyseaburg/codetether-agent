//! Scheduler state and constructors.

use std::collections::{HashMap, VecDeque};

use crate::scheduler::task::Task;

/// Dependency-free cooperative task scheduler.
#[derive(Debug)]
pub struct Scheduler {
    pub(super) tasks: HashMap<u64, Task>,
    pub(super) ready: VecDeque<u64>,
    pub(super) next_id: u64,
}

impl Scheduler {
    /// Create an empty scheduler.
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            ready: VecDeque::new(),
            next_id: 1,
        }
    }

    /// Pop the next runnable task id.
    pub fn next_ready(&mut self) -> Option<u64> {
        self.ready.pop_front()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
