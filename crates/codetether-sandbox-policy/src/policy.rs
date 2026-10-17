//! Execution permissions and resource budgets for one tool invocation.
use super::*;

/// Sandbox execution policy for a tool invocation.
///
/// # Examples
/// ```rust
/// let policy = codetether_sandbox_policy::SandboxPolicy::default();
/// assert!(!policy.allow_exec);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPolicy {
    /// Whether filesystem access is allowed (and to which paths).
    pub allowed_paths: Vec<PathBuf>,
    /// Whether network access is allowed.
    pub allow_network: bool,
    /// Whether shell execution is allowed.
    pub allow_exec: bool,
    /// Maximum execution time in seconds.
    pub timeout_secs: u64,
    /// Maximum memory in bytes (0 = no limit).
    pub max_memory_bytes: u64,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self {
            allowed_paths: Vec::new(),
            allow_network: false,
            allow_exec: false,
            timeout_secs: 30,
            max_memory_bytes: 0,
        }
    }
}
