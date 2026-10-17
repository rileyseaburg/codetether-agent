//! Browser control commands over the shared CodeTether browser service.
//!
//! [BrowserCtlTool] parses actions, builds typed browser requests and formats
//! their results. Screenshot responses retain image metadata while writing
//! their bytes to the caller's output path. Browser session ownership stays
//! with the shared browser runtime.
//!
//! ```rust
//! let _tool = codetether_browser_tool::BrowserCtlTool::new();
//! ```
mod actions;
mod detect;
mod dispatch;
mod helpers;
mod input;
mod response;
mod schema;
mod screenshot;
mod tool;
use codetether_tool_core::{Tool, ToolResult};
/// Browser automation tool using the shared browser session.
///
/// # Examples
/// ```rust
/// let _tool = codetether_browser_tool::BrowserCtlTool::new();
/// ```
pub struct BrowserCtlTool;
impl BrowserCtlTool {
    /// Create the stateless browser-control tool.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool ready for registration.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_browser_tool::BrowserCtlTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
impl Default for BrowserCtlTool {
    fn default() -> Self {
        Self::new()
    }
}
