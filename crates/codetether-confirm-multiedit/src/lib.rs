//! Ordered multi-file editing with an explicit confirmation step.
//!
//! [ConfirmMultiEditTool] validates every operation and prepares [EditPreview]
//! records before considering confirmation. [EditOperation] describes a requested
//! replacement. Accepted operations are written in their original order.
//!
//! ```rust
//! let _tool = codetether_confirm_multiedit::ConfirmMultiEditTool::new();
//! ```
use codetether_tool_core::{Tool, ToolResult};
mod apply;
mod array;
mod diff;
mod execute;
mod file;
mod input;
mod load;
mod new_string;
mod old_string;
mod prepare;
mod records;
mod review;
mod schema;
mod tool;
mod truncate;
mod unique;
pub use records::{EditOperation, EditPreview};
/// Preview and explicitly accept or reject an ordered group of replacements.
///
/// # Examples
/// ```rust
/// let _tool = codetether_confirm_multiedit::ConfirmMultiEditTool::new();
/// ```
pub struct ConfirmMultiEditTool;
impl ConfirmMultiEditTool {
    /// Create the stateless multi-file confirmation tool.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_confirm_multiedit::ConfirmMultiEditTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
impl Default for ConfirmMultiEditTool {
    fn default() -> Self {
        Self::new()
    }
}
