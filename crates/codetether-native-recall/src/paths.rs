//! Native recall catalog path resolution.
pub use codetether_recall_store::paths::canonical;
/// Resolve the catalog path for a canonical workspace.
/// # Arguments
/// * `workspace` - Workspace whose recall catalog is requested.
/// # Returns
/// The native catalog path.
/// # Errors
/// Returns data-directory resolution errors.
/// # Examples
/// `catalog::<S>(workspace)?` resolves the catalog for session type S.
pub fn catalog<S: crate::Session>(
    workspace: &std::path::Path,
) -> anyhow::Result<std::path::PathBuf> {
    codetether_recall_store::paths::catalog(workspace, &crate::Host::<S>::default())
}
