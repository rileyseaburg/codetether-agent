//! Mock actions injected during a worker model call.

use std::sync::atomic::{AtomicBool, AtomicUsize};
#[path = "session_control_actions.rs"]
mod actions;
#[path = "session_control_observe.rs"]
mod observe;
#[path = "session_control_provider.rs"]
mod provider;
#[path = "session_control_responses.rs"]
mod responses;

#[derive(Clone, Copy)]
pub(super) enum Scenario {
    Steering,
    Cancel,
    Pause,
    Hold,
    Goal,
}

pub(super) struct Mock {
    id: String,
    scenario: Scenario,
    calls: AtomicUsize,
    saw_input: AtomicBool,
}

impl Mock {
    pub(super) fn new(id: &str, scenario: Scenario) -> Self {
        Self {
            id: id.into(),
            scenario,
            calls: AtomicUsize::new(0),
            saw_input: AtomicBool::new(false),
        }
    }
    pub(super) fn outcome(&self) -> (usize, bool) {
        use std::sync::atomic::Ordering;
        (
            self.calls.load(Ordering::SeqCst),
            self.saw_input.load(Ordering::SeqCst),
        )
    }
}
