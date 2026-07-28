//! Deterministic browser timer queues.

use std::collections::{HashSet, VecDeque};

use super::ScheduledCallback;

struct TimedCallback {
    deadline_ms: u64,
    order: u64,
    delay_ms: u64,
    task: ScheduledCallback,
}

#[derive(Default)]
pub(super) struct TimerQueue {
    pub(super) next_id: u32,
    now_ms: u64,
    next_order: u64,
    timers: Vec<TimedCallback>,
    pub(super) interval_ids: HashSet<u32>,
    pub(super) microtasks: VecDeque<ScheduledCallback>,
    pub(super) animation_frames: VecDeque<ScheduledCallback>,
    pub(super) idle_callbacks: VecDeque<ScheduledCallback>,
}

impl TimerQueue {
    pub(super) fn schedule_timeout(&mut self, delay_ms: u64, task: ScheduledCallback) {
        self.insert_timer(delay_ms, task);
    }

    pub(super) fn cancel_timer(&mut self, id: u32) {
        self.interval_ids.remove(&id);
        self.timers.retain(|task| task.task.id != id);
    }

    pub(super) fn pop_timer(&mut self) -> Option<(ScheduledCallback, u64)> {
        let timed = self.timers.first()?;
        self.now_ms = timed.deadline_ms;
        let timed = self.timers.remove(0);
        Some((timed.task, timed.delay_ms))
    }

    pub(super) fn reschedule_interval(&mut self, task: ScheduledCallback, delay_ms: u64) {
        self.insert_timer(delay_ms, task);
    }

    fn insert_timer(&mut self, delay_ms: u64, task: ScheduledCallback) {
        let timed = TimedCallback {
            deadline_ms: self.now_ms.saturating_add(delay_ms),
            order: self.next_order,
            delay_ms,
            task,
        };
        self.next_order = self.next_order.saturating_add(1);
        let index = self.timers.partition_point(|queued| {
            (queued.deadline_ms, queued.order) <= (timed.deadline_ms, timed.order)
        });
        self.timers.insert(index, timed);
    }
}
