/// Structured tool failure fields extracted for display.
///
/// # Examples
/// ```
/// use codetether_tool_panel::error::StructuredToolError;
/// let error = StructuredToolError { code: None, tool: None, message: None, missing_fields: vec![], example: None };
/// assert!(error.missing_fields.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredToolError {
    /// Optional machine-readable error code.
    pub code: Option<String>,
    /// Optional tool name.
    pub tool: Option<String>,
    /// Optional failure description.
    pub message: Option<String>,
    /// Required fields reported as missing.
    pub missing_fields: Vec<String>,
    /// Pretty-printed JSON example, when supplied.
    pub example: Option<String>,
}
