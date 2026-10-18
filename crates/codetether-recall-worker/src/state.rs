//! Process-wide queue state and its original poison-tolerant mutex.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub(super) struct State {
    pub(super) pending: Option<String>,
    pub(super) running: bool,
}

pub(super) fn states() -> &'static Mutex<HashMap<String, State>> {
    static STATES: OnceLock<Mutex<HashMap<String, State>>> = OnceLock::new();
    STATES.get_or_init(|| Mutex::new(HashMap::new()))
}
