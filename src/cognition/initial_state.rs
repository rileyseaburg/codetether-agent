//! Native persistence selection for initial cognition state.
pub(super) use codetether_cognition_core::runtime::InitialState;
/// Tests always start from empty state so they never read the shared store.
#[cfg(test)]
pub(super) fn load_initial_state() -> InitialState {
    InitialState::default()
}

/// Load persisted state before construction. Doing this eagerly avoids a
/// `blocking_write()` inside a tokio runtime, which would panic.
#[cfg(not(test))]
pub(super) fn load_initial_state() -> InitialState {
    let Some(persisted) = super::persistence::load_state() else {
        return InitialState::default();
    };
    tracing::info!(
        personas = persisted.personas.len(),
        beliefs = persisted.beliefs.len(),
        persisted_at = %persisted.persisted_at,
        "Restoring persisted cognition state"
    );
    InitialState {
        personas: persisted.personas,
        beliefs: persisted.beliefs,
        proposals: persisted.proposals,
        attention: persisted.attention_queue,
        workspace: persisted.workspace,
    }
}
