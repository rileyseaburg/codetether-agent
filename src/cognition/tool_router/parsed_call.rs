//! Parsed tool-call contract from FunctionGemma output.

/// A single parsed tool call from FunctionGemma output.
#[derive(Debug, Clone)]
pub(super) struct ParsedToolCall {
    pub name: String,
    /// Arguments as a JSON object string.
    pub arguments: String,
}
