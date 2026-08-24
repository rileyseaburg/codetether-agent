//! Lock-protected state container for command-prefix grants.

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, MutexGuard, OnceLock};

type Request = (String, String, Vec<String>);

#[derive(Default)]
pub(super) struct State {
    pub(super) requests: HashMap<String, Request>,
    pub(super) allowed: HashSet<(String, String, String)>,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

pub(super) fn state() -> MutexGuard<'static, State> {
    STATE
        .get_or_init(|| Mutex::new(State::default()))
        .lock()
        .expect("session command grants lock")
}
