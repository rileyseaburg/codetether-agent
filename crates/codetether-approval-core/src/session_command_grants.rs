//! Process-local command-prefix grants for the current session.

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Default)]
struct State {
    requests: HashMap<String, Vec<String>>,
    allowed: HashSet<String>,
}

static STATE: OnceLock<Mutex<State>> = OnceLock::new();

fn state_cell() -> &'static Mutex<State> {
    STATE.get_or_init(|| Mutex::new(State::default()))
}

fn state() -> MutexGuard<'static, State> {
    state_cell().lock().expect("session command grants lock")
}

/// Store normalized command prefixes proposed by one approval request.
///
/// # Panics
/// Panics if the command grant mutex was poisoned.
pub fn remember_request(id: &str, prefixes: Vec<String>) {
    let prefixes = prefixes
        .into_iter()
        .filter_map(|p| normalize(&p))
        .collect::<Vec<_>>();
    if prefixes.is_empty() {
        return;
    }
    state().requests.insert(id.to_string(), prefixes);
}

/// Consume a request and grant every prefix it proposed.
///
/// # Panics
/// Panics if the command grant mutex was poisoned.
pub fn grant_for_request(id: &str) {
    let mut state = state();
    if let Some(prefixes) = state.requests.remove(id) {
        state.allowed.extend(prefixes);
    }
}

/// Check whether a command matches a granted prefix at a word boundary.
///
/// # Panics
/// Panics if the command grant mutex was poisoned.
pub fn allowed(command: &str) -> bool {
    let command = command.trim_start();
    state()
        .allowed
        .iter()
        .any(|p| command == p || command.starts_with(&format!("{p} ")))
}

fn normalize(prefix: &str) -> Option<String> {
    let prefix = prefix.trim();
    (!prefix.is_empty() && !prefix.contains(['\n', '\r'])).then(|| prefix.to_string())
}

#[cfg(any(test, feature = "test-support"))]
/// Clear request proposals and grants for tests using the test-support feature.
///
/// # Panics
/// Panics if the command grant mutex was poisoned.
pub fn reset() {
    let mut state = state();
    state.requests.clear();
    state.allowed.clear();
}
