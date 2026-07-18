//! Open and closed transitions within the execution registry.

use super::registry::STATE;

pub(in crate::tool::agent) fn close(name: &str) -> bool {
    let handle = STATE.lock().ok().and_then(|mut state| {
        state.closed.insert(name.into());
        state.aborts.remove(name)
    });
    handle.is_some_and(|handle| {
        handle.abort();
        true
    })
}

pub(in crate::tool::agent) fn reopen(name: &str) {
    if let Ok(mut state) = STATE.lock() {
        state.closed.remove(name);
    }
}

pub(in crate::tool::agent) fn is_closed(name: &str) -> bool {
    STATE.lock().is_ok_and(|state| state.closed.contains(name))
}
