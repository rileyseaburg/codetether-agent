//! Open and closed transitions within the execution registry.

use super::registry::STATE;

/// Mark an agent closed and cancel its registered turn.
/// # Arguments
/// * `name` — Agent execution identity.
/// # Returns
/// True if a registered handle was cancelled.
/// # Examples
/// ```
/// let _=codetether_collaboration_state::execution::close("docs-close");
/// ```
pub fn close(name: &str) -> bool {
    let handle = STATE.lock().ok().and_then(|mut state| {
        state.closed.insert(name.into());
        state.aborts.remove(name)
    });
    handle.is_some_and(|handle| {
        handle.abort();
        true
    })
}

/// Allow a previously closed agent to start another turn.
/// # Arguments
/// * `name` — Agent execution identity.
/// # Returns
/// Returns after clearing the closed flag when the registry lock is available.
/// # Examples
/// ```
/// codetether_collaboration_state::execution::reopen("docs-close");
/// ```
pub fn reopen(name: &str) {
    if let Ok(mut state) = STATE.lock() {
        state.closed.remove(name);
    }
}

/// Check the agent execution closed flag.
/// # Arguments
/// * `name` — Agent execution identity.
/// # Returns
/// True if closed; false if absent or the registry lock is poisoned.
/// # Examples
/// ```
/// let _=codetether_collaboration_state::execution::is_closed("docs-agent");
/// ```
pub fn is_closed(name: &str) -> bool {
    STATE.lock().is_ok_and(|state| state.closed.contains(name))
}
