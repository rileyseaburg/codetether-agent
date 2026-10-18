use crate::{AttentionItem, GlobalWorkspace, PersonaRuntimeState, Proposal, beliefs::Belief};
use std::collections::HashMap;
/// State a new runtime starts from, either empty or restored from disk.
/// # Examples
/// ```
/// let initial = codetether_cognition_core::runtime::InitialState::default();
/// assert!(initial.personas.is_empty());
/// ```
pub struct InitialState {
    /// Restored persona states.
    pub personas: HashMap<String, PersonaRuntimeState>,
    /// Restored beliefs.
    pub beliefs: HashMap<String, Belief>,
    /// Restored proposals.
    pub proposals: HashMap<String, Proposal>,
    /// Restored attention queue.
    pub attention: Vec<AttentionItem>,
    /// Restored shared workspace.
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
