//! Native actions used by session utility command routing.
use std::future::Future;
/// Application-bound actions invoked by utility command policy.
/// # Implementors
/// Frontends provide session/workspace context to their existing handlers.
pub trait Host {
    /// Handle an access-mode command before the utility routes.
    fn access_mode(&mut self, prompt: &str) -> impl Future<Output = bool>;
    /// Show a status report with the requested heading.
    fn status(&mut self, label: &str) -> impl Future<Output = ()>;
    /// Show the working-tree diff.
    fn diff(&mut self);
    /// Copy a selected message payload.
    fn copy(&mut self, prompt: &str);
    /// Populate an editable review prompt.
    fn review(&mut self);
    /// Invoke the regular slash-command handler for an alias.
    fn dispatch(&mut self, command: &str) -> impl Future<Output = ()>;
    /// Clear composer input after alias dispatch.
    fn clear_input(&mut self);
}
