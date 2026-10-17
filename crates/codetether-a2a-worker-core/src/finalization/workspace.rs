//! Workspace inputs required by post-agent finalization.
/// Borrowed repository path and execution attribution.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::finalization::FinalizationWorkspace;
/// let workspace = FinalizationWorkspace { directory: None, provenance: None };
/// assert!(workspace.directory.is_none());
/// ```
pub struct FinalizationWorkspace<'a> {
    /// Checkout directory, if the task has one.
    pub directory: Option<&'a std::path::Path>,
    /// Attribution to attach to a publication commit.
    pub provenance: Option<&'a codetether_provenance::ExecutionProvenance>,
}
