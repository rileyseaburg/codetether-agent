//! Native provider, session and view boundaries for interactive goal commands.
use crate::PendingOkrApproval;
use codetether_okr::OkrRepository;
use codetether_provider_core::Provider;
use std::{future::Future, sync::Arc};
/// Provider lookup operations used when drafting an approval objective.
/// # Implementors
/// Native applications supply their existing shared registry and model resolver.
pub trait DraftHost {
    /// Application-owned provider registry.
    type Registry;
    /// Load providers, returning none when they are unavailable.
    fn load_registry() -> impl Future<Output = Option<Arc<Self::Registry>>>;
    /// Resolve a relay provider and its model name using current policy.
    fn resolve(registry: &Arc<Self::Registry>, model: &str) -> Option<(Arc<dyn Provider>, String)>;
}
/// Session metadata and view updates used by the interactive goal command.
/// # Implementors
/// Native TUI bindings forward the active session and approval view fields.
pub trait CommandHost: DraftHost {
    /// Return the current session model, if set.
    fn current_model(&self) -> Option<&str>;
    /// Update the session model before drafting.
    fn set_model(&mut self, model: String);
    /// Persist the selected model, returning a displayable failure.
    fn save_session(&self) -> impl Future<Output = Result<(), String>>;
    /// Access the optional repository initialized for the approval flow.
    fn repository(&mut self) -> &mut Option<Arc<OkrRepository>>;
    /// Append the approval prompt to the interactive conversation.
    fn push_system_message(&mut self, message: String);
    /// Store the pending objective for later user approval.
    fn set_pending(&mut self, pending: PendingOkrApproval);
    /// Replace the interactive status text.
    fn set_status(&mut self, status: String);
}
