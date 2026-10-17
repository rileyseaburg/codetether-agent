//! Plugin signing, content verification, and subprocess policy checks.
//!
//! Plugin registration fails closed unless the manifest signature is valid and
//! the declared content hash matches the bytes or file supplied by the caller.
//! Subprocess execution enforces pre-spawn policy checks and reports any
//! platform fallback that is only advisory.
//!
//! Built-in tools are trusted. Third-party plugins must be registered with
//! verified content before callers should execute them.

use serde::{Deserialize, Serialize};

use std::path::PathBuf;

include!("sandbox_modules.rs");

pub use sandbox_availability::{direct_fallback_env_allowed, unavailable_reason};

pub use sandbox_execute::execute_sandboxed;

/// Sandbox execution policy for a tool invocation.
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

/// Result of a sandboxed tool execution.
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
pub use codetether_plugin_verification::{
    PluginManifest, PluginRegistry, SigningKey, hash_bytes, hash_file,
};
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sandbox_denies_working_dir_outside_allowed_paths() {
        let allowed = tempfile::tempdir().expect("allowed tempdir");
        let denied = tempfile::tempdir().expect("denied tempdir");
        let policy = SandboxPolicy {
            allowed_paths: vec![allowed.path().to_path_buf()],
            allow_network: true,
            allow_exec: true,
            ..SandboxPolicy::default()
        };

        let err = execute_sandboxed(
            "sh",
            &["-c".to_string(), "echo denied".to_string()],
            &policy,
            Some(denied.path()),
        )
        .await
        .expect_err("denied path should fail closed");
        assert!(err.to_string().contains("denied working directory"));
    }

    #[test]
    fn sandbox_memory_limit_is_opt_in_by_default() {
        assert_eq!(SandboxPolicy::default().max_memory_bytes, 0);
    }

    #[tokio::test]
    async fn sandbox_denies_network_commands_when_network_disabled() {
        let policy = SandboxPolicy {
            allow_network: false,
            allow_exec: true,
            ..SandboxPolicy::default()
        };
        let err = execute_sandboxed(
            "sh",
            &["-c".to_string(), "curl https://example.com".to_string()],
            &policy,
            None,
        )
        .await
        .expect_err("network command should fail closed");
        assert!(err.to_string().contains("denies network access"));
    }

    #[tokio::test]
    async fn sandbox_denies_dequoted_network_commands() {
        let policy = SandboxPolicy {
            allow_network: false,
            allow_exec: true,
            ..SandboxPolicy::default()
        };
        let err = execute_sandboxed(
            "sh",
            &["-c".to_string(), "c\"\"url https://example.com".to_string()],
            &policy,
            None,
        )
        .await
        .expect_err("dequoted network command should fail closed");
        assert!(err.to_string().contains("denies network access"));
    }
}
