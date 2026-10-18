//! Mutable dashboard operations used by Enter-key navigation.
use codetether_swarm_view::SwarmViewState;
/// Native selection, registry and presentation operations.
/// # Implementors
/// Frontends retain registry access while this crate owns navigation policy.
pub trait Host {
    /// Borrow the focused agent name.
    fn selected(&self) -> Option<&str>;
    /// Clear a selected name that no longer resolves.
    fn clear_selected(&mut self);
    /// Test membership in the local agent registry.
    fn local_contains(&self, name: &str) -> bool;
    /// Test membership in the current session's tool-agent registry.
    fn owned_tool_contains(&self, name: &str) -> bool;
    /// Borrow swarm selection and subtasks.
    fn swarm(&self) -> &SwarmViewState;
    /// Update swarm selection and detail mode.
    fn swarm_mut(&mut self) -> &mut SwarmViewState;
    /// Switch the frontend to the swarm view.
    fn show_swarm(&mut self);
    /// Open local-agent detail mode and reset its scroll offset.
    fn open_local_detail(&mut self);
    /// Cycle to the next available agent using the frontend's navigation source.
    fn cycle_focus(&mut self);
    /// Replace visible status text.
    fn set_status(&mut self, status: String);
}
