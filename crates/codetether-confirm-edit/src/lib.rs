//! Single-file edits with a preview and explicit confirmation.
//!
//! [ConfirmEditTool] validates a unique match and returns a colored diff when
//! confirmation is omitted. Confirmed writes retain file-change telemetry.
//!
//! ```rust
//! let _tool = codetether_confirm_edit::ConfirmEditTool::new();
//! ```
use codetether_tool_core::{Tool, ToolResult};
mod apply;
mod diff;
mod execute;
mod input;
mod new_string;
mod old_string;
mod path;
mod preview;
mod schema;
mod telemetry;
mod tool;
mod validate;
/// Preview, confirm or reject a uniquely matched file replacement.
///
/// # Examples
/// ```rust
/// let _tool = codetether_confirm_edit::ConfirmEditTool::new();
/// ```
pub struct ConfirmEditTool;
impl ConfirmEditTool {
    /// Construct the stateless confirmation tool.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_confirm_edit::ConfirmEditTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
impl Default for ConfirmEditTool {
    fn default() -> Self {
        Self::new()
    }
}
