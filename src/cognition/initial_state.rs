//! Initial state restored at runtime construction.

use std::collections::HashMap;

use super::beliefs::Belief;
use super::{AttentionItem, GlobalWorkspace, PersonaRuntimeState, Proposal};

/// State a new runtime starts from, either empty or restored from disk.
pub(super) struct InitialState {
    pub personas: HashMap<String, PersonaRuntimeState>,
    pub beliefs: HashMap<String, Belief>,
    pub proposals: HashMap<String, Proposal>,
    pub attention: Vec<AttentionItem>,
    pub workspace: GlobalWorkspace,
}

impl Default for InitialState {
    fn default() -> Self {
        Self {
            personas: HashMap::new(),
            beliefs: HashMap::new(),
            proposals: HashMap::new(),
            attention: Vec::new(),
            workspace: GlobalWorkspace::default(),
        }
    }
}

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
