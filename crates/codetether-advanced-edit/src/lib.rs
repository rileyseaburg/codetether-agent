//! File editing with an ordered set of exact and whitespace-tolerant matchers.
//!
//! [AdvancedEditTool] owns argument validation and asynchronous file access.
//! Private strategy modules find candidates; replacement rejects ambiguous matches.
//!
//! ```rust
//! let _tool = codetether_advanced_edit::AdvancedEditTool::new();
//! ```
use codetether_tool_core::{Tool, ToolResult};
mod block_anchor;
mod block_best;
mod boundary;
mod execute;
mod file_path;
mod indentation;
mod input;
mod levenshtein;
mod line_trimmed;
mod new_string;
mod old_string;
mod path_check;
mod prepare;
mod replace;
mod schema;
mod simple;
mod tool;
mod whitespace;
/// Replaces file text using ordered matching strategies and ambiguity checks.
///
/// # Examples
/// ```rust
/// let _tool = codetether_advanced_edit::AdvancedEditTool::new();
/// ```
pub struct AdvancedEditTool;
impl AdvancedEditTool {
    /// Create the stateless editing tool.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_advanced_edit::AdvancedEditTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
impl Default for AdvancedEditTool {
    fn default() -> Self {
        Self::new()
    }
}
