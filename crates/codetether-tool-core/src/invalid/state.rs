//! Invalid-tool fallback context and construction.

/// Invalid Tool - A fallback tool that is returned when a model tries to call
/// a tool that doesn't exist. This provides helpful error messages and suggestions.
/// Fallback that returns corrective feedback for unknown tool identifiers.
///
/// # Examples
/// ```
/// use codetether_tool_core::{InvalidTool, Tool};
/// assert_eq!(InvalidTool::new().id(), "invalid");
/// ```
pub struct InvalidTool {
    pub(super) requested_tool: String,
    pub(super) available_tools: Vec<String>,
}

impl Default for InvalidTool {
    fn default() -> Self {
        Self::new()
    }
}

impl InvalidTool {
    /// Construct fallback feedback without a catalog.
    ///
    /// # Examples
    /// ```
    /// use codetether_tool_core::{InvalidTool, Tool};
    /// assert_eq!(InvalidTool::new().name(), "Invalid Tool Handler");
    /// ```
    pub fn new() -> Self {
        Self {
            requested_tool: String::new(),
            available_tools: Vec::new(),
        }
    }

    /// Construct fallback feedback with the requested identifier and available tools.
    ///
    /// # Arguments
    /// * `requested_tool` - Fallback name when invocation arguments omit it.
    /// * `available_tools` - Ordered catalog used for suggestions.
    ///
    /// # Returns
    /// A fallback tool preserving the supplied catalog order.
    ///
    /// # Examples
    /// ```
    /// use codetether_tool_core::{InvalidTool, Tool};
    /// let tool = InvalidTool::with_context("raad".into(), vec!["read".into()]);
    /// assert_eq!(tool.id(), "invalid");
    /// ```
    pub fn with_context(requested_tool: String, available_tools: Vec<String>) -> Self {
        Self {
            requested_tool,
            available_tools,
        }
    }
}
