//! Classification of tools whose proposed file contents can be reconstructed.
//!
//! Concrete reconstruction and filesystem access are supplied by tool implementations.

/// Whether a tool supports reconstruction of its proposed file contents.
///
/// # Arguments
/// * `tool` — Registered tool identifier.
/// # Returns
/// Whether the standard tool implementation supports file-content previews.
/// # Examples
/// ```
/// assert!(codetether_tool_core::proposed::supported("write"));
/// assert!(!codetether_tool_core::proposed::supported("read"));
/// ```
pub fn supported(tool: &str) -> bool {
    matches!(
        tool,
        "apply_patch" | "patch" | "write" | "edit" | "multiedit"
    )
}
