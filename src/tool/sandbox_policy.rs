//! Filesystem, network, and process limits for sandboxed commands.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Authority granted to one sandboxed process.
///
/// Metadata paths are protected by default. Trusted repository-management
/// operations can explicitly disable that protection while retaining the
/// workspace and network sandbox.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::sandbox::SandboxPolicy;
///
/// let policy = SandboxPolicy::default();
/// assert!(!policy.allow_network);
/// assert!(policy.protect_metadata);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPolicy {
    /// Writable filesystem roots.
    pub allowed_paths: Vec<PathBuf>,
    /// Additional roots exposed read-only to the child.
    #[serde(default)]
    pub read_only_paths: Vec<PathBuf>,
    /// Explicit environment additions for the restricted child.
    #[serde(default)]
    pub environment: HashMap<String, String>,
    /// Whether network access is allowed.
    pub allow_network: bool,
    /// Whether process execution is allowed.
    pub allow_exec: bool,
    /// Maximum execution time in seconds.
    pub timeout_secs: u64,
    /// Maximum memory in bytes (`0` means unlimited).
    pub max_memory_bytes: u64,
    /// Keep repository and agent metadata read-only.
    #[serde(default = "enabled")]
    pub protect_metadata: bool,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self {
            allowed_paths: Vec::new(),
            read_only_paths: Vec::new(),
            environment: HashMap::new(),
            allow_network: false,
            allow_exec: false,
            timeout_secs: 30,
            max_memory_bytes: 0,
            protect_metadata: true,
        }
    }
}

fn enabled() -> bool {
    true
}
