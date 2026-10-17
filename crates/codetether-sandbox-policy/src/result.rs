//! Outcome and confinement evidence for a sandboxed tool execution.
use super::*;

/// Result of a sandboxed tool execution.
///
/// # Examples
/// ```rust
/// let result = codetether_sandbox_policy::SandboxResult {
///     tool_id: "example".into(), success: true, output: String::new(),
///     output_hash: String::new(), exit_code: Some(0), duration_ms: 0,
///     sandbox_violations: vec![], unsafe_fallbacks: vec![],
/// };
/// assert!(result.success);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxResult {
    pub tool_id: String,
    pub success: bool,
    pub output: String,
    /// SHA-256 hash of the combined output for integrity verification.
    pub output_hash: String,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
    pub sandbox_violations: Vec<String>,
    /// Isolation gaps that are visible to callers instead of hidden.
    #[serde(default)]
    pub unsafe_fallbacks: Vec<String>,
}
