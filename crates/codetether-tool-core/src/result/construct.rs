//! Success, failure, and metadata construction helpers.
use super::ToolResult;
use serde_json::Value;
use std::collections::HashMap;
impl ToolResult {
    /// Create a successful result with no metadata.
    ///
    /// # Arguments
    /// Text produced by the tool.
    ///
    /// # Returns
    /// The constructed or updated result.
    ///
    /// # Examples
    /// ```
    /// use codetether_tool_core::ToolResult;
    /// assert!(ToolResult::success("done").success);
    /// ```
    pub fn success(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            success: true,
            metadata: HashMap::new(),
        }
    }

    /// Create an unsuccessful result with no metadata.
    ///
    /// # Arguments
    /// Human-readable failure text.
    ///
    /// # Returns
    /// The constructed or updated result.
    ///
    /// # Examples
    /// ```
    /// use codetether_tool_core::ToolResult;
    /// assert!(!ToolResult::error("failed").success);
    /// ```
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            output: message.into(),
            success: false,
            metadata: HashMap::new(),
        }
    }

    /// Insert or replace one metadata key and return the result.
    ///
    /// # Arguments
    /// A metadata key and its JSON value.
    ///
    /// # Returns
    /// The constructed or updated result.
    ///
    /// # Examples
    /// ```
    /// use codetether_tool_core::ToolResult;
    /// let result = ToolResult::success("done").with_metadata("count", serde_json::json!(1));
    /// assert_eq!(result.metadata["count"], 1);
    /// ```
    pub fn with_metadata(mut self, key: impl Into<String>, value: Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}
