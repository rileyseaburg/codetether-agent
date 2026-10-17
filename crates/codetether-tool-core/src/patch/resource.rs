//! Stable approval identity for ordered patch target files.

/// Format the approval identity for an ordered set of patch targets.
///
/// # Arguments
/// * `files` — Target paths in the caller's stable order.
/// # Returns
/// Workspace for no targets, the single path, or comma-joined paths.
/// # Examples
/// ```
/// assert_eq!(codetether_tool_core::patch::approval_resource(&[]), "workspace");
/// ```
pub fn resource(files: &[String]) -> String {
    match files {
        [] => "workspace".to_string(),
        [file] => file.clone(),
        many => many.join(","),
    }
}
