//! UTF-8-safe result prefix budgeting with an explicit truncation marker.
use super::ToolResult;
impl ToolResult {
    /// Keep a UTF-8-safe output prefix and append a truncation marker if needed.
    ///
    /// # Arguments
    /// * `max_bytes` - Maximum retained prefix length; the marker is additional text.
    ///
    /// # Returns
    /// The result, preserving other metadata and adding truncation information only
    /// when output exceeded the limit. `shown_bytes` records the requested budget.
    ///
    /// # Examples
    /// ```
    /// use codetether_tool_core::ToolResult;
    /// let result = ToolResult::success("hello").truncate_to(3);
    /// assert!(result.output.starts_with("hel\n"));
    /// assert_eq!(result.metadata["truncated"]["original_bytes"], 5);
    /// ```
    pub fn truncate_to(mut self, max_bytes: usize) -> Self {
        if self.output.len() <= max_bytes {
            return self;
        }
        let original_len = self.output.len();
        let head = codetether_util::truncate::truncate_bytes_safe(&self.output, max_bytes);
        self.output =
            format!("{head}\n…[truncated: {original_len} bytes, showing first {max_bytes}]");
        self.metadata.insert(
            "truncated".to_string(),
            serde_json::json!({
                "original_bytes": original_len,
                "shown_bytes": max_bytes,
            }),
        );
        self
    }
}
