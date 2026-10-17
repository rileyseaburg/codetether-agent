//! Plugin signing, content verification, and subprocess policy checks.
//!
//! Plugin registration fails closed unless the manifest signature is valid and
//! the declared content hash matches the bytes or file supplied by the caller.
//! Subprocess execution enforces pre-spawn policy checks and reports any
//! platform fallback that is only advisory.
//!
//! Built-in tools are trusted. Third-party plugins must be registered with
//! verified content before callers should execute them.

include!("sandbox_modules.rs");

pub use sandbox_availability::{direct_fallback_env_allowed, unavailable_reason};

pub use codetether_plugin_verification::{
    PluginManifest, PluginRegistry, SigningKey, hash_bytes, hash_file,
};
pub use sandbox_execute::execute_sandboxed;
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

pub use codetether_sandbox_policy::{SandboxPolicy, SandboxResult};
