//! Shared workspace entry pruning policy.
/// Whether a directory name should be omitted from workspace listings.
/// # Arguments
/// * `name` - Directory entry name.
/// # Returns
/// Whether the shared workspace pruning policy excludes it.
/// # Examples
/// ```
/// assert!(codetether_workspace_snapshot::should_skip_entry("node_modules"));
/// ```
pub fn should_skip_entry(name: &str) -> bool {
    codetether_util::workspace_scan::is_pruned_workspace_dir(name)
}
