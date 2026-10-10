use super::state::State;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, atomic::AtomicBool};

/// Window-owned state remains address-stable until after native destruction.
pub(super) struct Context {
    pub(super) state: RefCell<State>,
    pub(super) invalidate: Cell<bool>,
    pub(super) interrupt: Arc<AtomicBool>,
    pub(super) session_blocked: Cell<bool>,
    pub(super) command: Cell<Option<usize>>,
    pub(super) relayout: Cell<bool>,
    pub(super) failure: RefCell<Option<anyhow::Error>>,
}

impl Context {
    pub(super) fn new() -> Self {
        let interrupt = Arc::new(AtomicBool::new(true));
        Self {
            state: RefCell::new(State::new(interrupt.clone())),
            interrupt,
            session_blocked: Cell::new(false),
            command: Cell::new(None),
            invalidate: Cell::new(false),
            relayout: Cell::new(false),
            failure: RefCell::new(None),
        }
    }
}
