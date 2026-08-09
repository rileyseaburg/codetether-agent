//! Runtime state for one mux-owned agent turn.

use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};

use super::buffer::TaskBuffer;

pub(super) struct AgentTask {
    pub(super) output: Mutex<TaskBuffer>,
    pub(super) changed: tokio::sync::watch::Sender<u64>,
    running: AtomicBool,
    pub(super) exit_code: Mutex<Option<i32>>,
    pub(super) pid: u32,
}

impl AgentTask {
    pub(super) fn new(pid: u32) -> Self {
        let (changed, _) = tokio::sync::watch::channel(0);
        Self {
            output: Mutex::new(TaskBuffer::new()),
            changed,
            running: AtomicBool::new(true),
            exit_code: Mutex::new(None),
            pid,
        }
    }

    pub(super) fn append(&self, data: &[u8]) {
        let offset = self.output.lock().unwrap().append(data);
        self.changed.send_replace(offset);
    }

    pub(super) fn finish(&self, exit_code: i32) {
        *self.exit_code.lock().unwrap() = Some(exit_code);
        self.running.store(false, Ordering::Release);
        self.changed
            .send_modify(|offset| *offset = offset.saturating_add(1));
    }

    pub(super) fn running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }
}
