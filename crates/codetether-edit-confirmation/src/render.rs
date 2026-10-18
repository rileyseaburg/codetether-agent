//! Stable user-visible status for pending and automatically confirmed edits.

/// Reminder appended to tool output when `auto_apply_edits` is **off** and the
/// tool produced only a preview.
/// # Arguments
/// Receives the preview tool name and output.
/// # Returns
/// The preview output followed by its pending-confirmation status.
/// # Examples
/// ```
/// assert!(codetether_edit_confirmation::pending_confirmation_tool_result_content("edit", "preview").contains("NOT been applied"));
/// ```
pub fn pending_confirmation_tool_result_content(tool_name: &str, content: &str) -> String {
    format!(
        "{content}\n\nStatus: Pending confirmation only. `{tool_name}` has NOT been applied yet. \
         Auto-apply is off. Enable it in TUI Settings or with `/autoapply on` if you want pending \
         edit previews to be confirmed automatically."
    )
}

/// Banner prefix shown when `auto_apply_edits` is **on** and a pending change
/// has been automatically confirmed.
/// # Arguments
/// Receives the confirmation output and success flag.
/// # Returns
/// A status banner followed by the original output.
/// # Examples
/// ```
/// assert!(codetether_edit_confirmation::auto_apply_pending_confirmation_result_content("done", true).contains("automatically confirmed"));
/// ```
pub fn auto_apply_pending_confirmation_result_content(output: &str, success: bool) -> String {
    let status = if success {
        "TUI edit auto-apply is enabled. The pending change was automatically confirmed and applied."
    } else {
        "TUI edit auto-apply is enabled, but confirming the pending change failed."
    };
    format!("{status}\n\n{output}")
}
