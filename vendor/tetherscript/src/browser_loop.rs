//! Deterministic browser-style event loop queues.
//!
//! The loop models four queues commonly used by browser runtimes:
//! macrotasks, microtasks, animation-frame callbacks, and timers. It is
//! intentionally single-threaded and deterministic so embedding tests can assert
//! exact callback ordering.

use std::collections::VecDeque;

/// Callback stored by the deterministic browser loop.
pub type Callback = Box<dyn FnOnce(&mut BrowserLoop) + 'static>;

struct TimerTask {
    due_tick: u64,
    sequence: u64,
    callback: Callback,
}

/// A deterministic, single-threaded browser-style event loop.
///
/// Each [`run_ticks`](Self::run_ticks) tick performs one event-loop turn:
///
/// 1. due timers are promoted to the macrotask queue in deadline/sequence order;
/// 2. one macrotask is executed, then the microtask queue is drained;
/// 3. animation-frame callbacks queued before the frame phase are executed, with
///    microtasks drained after each callback;
/// 4. the logical tick counter advances by one.
///
/// [`run_until_idle`](Self::run_until_idle) repeats ticks until all queues are
/// empty, including future timers.
#[derive(Default)]
pub struct BrowserLoop {
    tick: u64,
    next_sequence: u64,
    macrotasks: VecDeque<Callback>,
    microtasks: VecDeque<Callback>,
    animation_frames: VecDeque<Callback>,
    timers: Vec<TimerTask>,
}

impl BrowserLoop {
    /// Create an empty event loop at logical tick zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the current logical tick.
    pub fn current_tick(&self) -> u64 {
        self.tick
    }

    /// Queue a macrotask.
    pub fn queue_macrotask<F>(&mut self, callback: F)
    where
        F: FnOnce(&mut BrowserLoop) + 'static,
    {
        self.macrotasks.push_back(Box::new(callback));
    }

    /// Queue a microtask.
    pub fn queue_microtask<F>(&mut self, callback: F)
    where
        F: FnOnce(&mut BrowserLoop) + 'static,
    {
        self.microtasks.push_back(Box::new(callback));
    }

    /// Queue an animation-frame callback.
    pub fn request_animation_frame<F>(&mut self, callback: F)
    where
        F: FnOnce(&mut BrowserLoop) + 'static,
    {
        self.animation_frames.push_back(Box::new(callback));
    }

    /// Queue a timer callback after `delay_ticks` logical ticks.
    ///
    /// Timers with the same deadline are executed in registration order. A zero
    /// delay timer is promoted to the macrotask queue at the start of the next
    /// processed turn.
    pub fn set_timeout<F>(&mut self, delay_ticks: u64, callback: F)
    where
        F: FnOnce(&mut BrowserLoop) + 'static,
    {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.timers.push(TimerTask {
            due_tick: self.tick.saturating_add(delay_ticks),
            sequence,
            callback: Box::new(callback),
        });
    }

    /// Run up to `ticks` event-loop turns.
    pub fn run_ticks(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.run_one_tick();
        }
    }

    /// Run event-loop turns until all queues, including future timers, are idle.
    pub fn run_until_idle(&mut self) {
        while self.has_pending_work() {
            self.run_one_tick();
        }
    }

    /// Return true when no queue contains pending work.
    pub fn is_idle(&self) -> bool {
        !self.has_pending_work()
    }

    fn has_pending_work(&self) -> bool {
        !(self.macrotasks.is_empty()
            && self.microtasks.is_empty()
            && self.animation_frames.is_empty()
            && self.timers.is_empty())
    }

    fn run_one_tick(&mut self) {
        self.promote_due_timers();

        if let Some(task) = self.macrotasks.pop_front() {
            task(self);
            self.drain_microtasks();
        } else if !self.microtasks.is_empty() {
            self.drain_microtasks();
        }

        let frames_this_tick = self.animation_frames.len();
        for _ in 0..frames_this_tick {
            if let Some(frame) = self.animation_frames.pop_front() {
                frame(self);
                self.drain_microtasks();
            }
        }

        self.tick = self.tick.saturating_add(1);
    }

    fn drain_microtasks(&mut self) {
        while let Some(task) = self.microtasks.pop_front() {
            task(self);
        }
    }

    fn promote_due_timers(&mut self) {
        self.timers
            .sort_by_key(|timer| (timer.due_tick, timer.sequence));

        let mut pending = Vec::new();
        for timer in self.timers.drain(..) {
            if timer.due_tick <= self.tick {
                self.macrotasks.push_back(timer.callback);
            } else {
                pending.push(timer);
            }
        }
        self.timers = pending;
    }
}

#[cfg(test)]
mod tests {
    use super::BrowserLoop;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn record(log: &Rc<RefCell<Vec<&'static str>>>, value: &'static str) {
        log.borrow_mut().push(value);
    }

    #[test]
    fn macrotask_microtask_and_animation_frame_ordering_is_deterministic() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut loop_ = BrowserLoop::new();

        let log_for_task = Rc::clone(&log);
        loop_.queue_macrotask(move |loop_| {
            record(&log_for_task, "macro-1");
            let log_for_micro = Rc::clone(&log_for_task);
            loop_.queue_microtask(move |_| record(&log_for_micro, "micro-after-macro-1"));
        });

        let log_for_frame = Rc::clone(&log);
        loop_.request_animation_frame(move |loop_| {
            record(&log_for_frame, "raf-1");
            let log_for_micro = Rc::clone(&log_for_frame);
            loop_.queue_microtask(move |_| record(&log_for_micro, "micro-after-raf-1"));
        });

        let log_for_task = Rc::clone(&log);
        loop_.queue_macrotask(move |_| record(&log_for_task, "macro-2"));

        loop_.run_until_idle();

        assert_eq!(
            *log.borrow(),
            vec![
                "macro-1",
                "micro-after-macro-1",
                "raf-1",
                "micro-after-raf-1",
                "macro-2",
            ]
        );
    }

    #[test]
    fn timers_are_ordered_by_deadline_then_registration_order() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut loop_ = BrowserLoop::new();

        for (delay, name) in [
            (2, "timer-2a"),
            (1, "timer-1"),
            (2, "timer-2b"),
            (0, "timer-0"),
        ] {
            let log_for_timer = Rc::clone(&log);
            loop_.set_timeout(delay, move |_| record(&log_for_timer, name));
        }

        loop_.run_until_idle();

        assert_eq!(
            *log.borrow(),
            vec!["timer-0", "timer-1", "timer-2a", "timer-2b"]
        );
    }

    #[test]
    fn run_ticks_processes_one_macrotask_turn_at_a_time() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut loop_ = BrowserLoop::new();

        let first = Rc::clone(&log);
        loop_.queue_macrotask(move |_| record(&first, "macro-1"));
        let second = Rc::clone(&log);
        loop_.queue_macrotask(move |_| record(&second, "macro-2"));

        loop_.run_ticks(1);
        assert_eq!(*log.borrow(), vec!["macro-1"]);
        assert_eq!(loop_.current_tick(), 1);

        loop_.run_ticks(1);
        assert_eq!(*log.borrow(), vec!["macro-1", "macro-2"]);
        assert!(loop_.is_idle());
    }
}
