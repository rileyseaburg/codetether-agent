//! Restore the initial runtime stores from the persisted cognition snapshot.
use codetether_cognition_core::runtime::InitialState;
/// Restore saved stores, falling back to an empty initial state.
/// # Returns
/// Saved personas, beliefs, proposals, attention and workspace when available.
/// # Examples
/// ```no_run
/// let initial = codetether_cognition_storage::load_initial_state();
/// # let _ = initial;
/// ```
pub fn load_initial_state() -> InitialState {
    let Some(persisted) = super::load_state() else {
        return InitialState::default();
    };
    tracing::info!(target: "codetether_agent::cognition::initial_state",
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
