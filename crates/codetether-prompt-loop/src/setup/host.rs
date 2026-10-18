//! Session and application services needed during prompt preparation.
use anyhow::Result;
use std::{
    collections::HashSet,
    future::Future,
    path::{Path, PathBuf},
};

/// Supplies session/model access while shared setup controls operation order.
/// # Implementors
/// Applications borrow their session, optional event channel, and provider
/// registry, and bind workspace inspection and optional tool routing.
pub trait Host: Send {
    /// Prepared provider/model state retained across workspace inspection.
    type Model: Send;
    /// Optional application response-to-tool-call router.
    type Router;
    /// Fresh sub-agent completion watcher.
    type Subagents: Default;
    /// Send the initial thinking notification, ignoring delivery failure.
    fn thinking(&mut self) -> impl Future<Output = ()> + Send;
    /// Resolve the subcall provider before preparing the initial model.
    /// # Errors
    /// Returns initial model/provider selection errors.
    fn resolve_model(&mut self) -> Result<Self::Model>;
    /// Clone the configured session directory, if any.
    fn directory(&self) -> Option<PathBuf>;
    /// Capture dirty paths before this turn makes changes.
    fn capture_dirty(&self, cwd: &Path) -> impl Future<Output = HashSet<PathBuf>> + Send;
    /// Optional step limit recorded on the session.
    fn max_steps(&self) -> Option<usize>;
    /// Session identity used in the per-turn lease owner.
    fn session_id(&self) -> &str;
    /// Construct the optional application tool-call router.
    fn router(&self) -> Option<Self::Router>;
}
