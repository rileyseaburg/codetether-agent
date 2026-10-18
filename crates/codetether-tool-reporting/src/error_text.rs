//! Preserve complete tool-failure cause chains.
//! Preserve the entire anyhow cause chain when presenting a tool failure.

/// # Arguments
/// The tool-execution error.
/// # Returns
/// A formatted string containing every attached context.
/// # Examples
/// ```
/// assert_eq!(codetether_tool_reporting::diagnostic(&anyhow::anyhow!("failed")), "failed");
/// ```
pub fn diagnostic(error: &anyhow::Error) -> String {
    format!("{error:#}")
}
