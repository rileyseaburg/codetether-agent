//! Per-poll head-tail output budgeting with omission accounting.

use std::collections::VecDeque;

#[path = "buffer/finish.rs"]
mod finish;
#[path = "buffer/push.rs"]
mod push;

pub(super) struct Buffer {
    pub(super) head: Vec<u8>,
    pub(super) tail: VecDeque<u8>,
    pub(super) head_budget: usize,
    pub(super) tail_budget: usize,
    pub(super) omitted: usize,
}

impl Buffer {
    pub fn new(max_bytes: usize) -> Self {
        let head_budget = max_bytes / 2;
        Self {
            head: Vec::with_capacity(head_budget),
            tail: VecDeque::with_capacity(max_bytes.saturating_sub(head_budget)),
            head_budget,
            tail_budget: max_bytes.saturating_sub(head_budget),
            omitted: 0,
        }
    }

    pub fn push(&mut self, chunk: &[u8]) {
        push::chunk(self, chunk);
    }

    pub fn finish(self) -> (String, usize) {
        finish::output(self)
    }
}

#[cfg(test)]
#[path = "buffer/tests.rs"]
mod tests;
