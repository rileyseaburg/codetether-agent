//! Minimal sandbox environment with explicitly exposed Rust toolchain homes.

use std::collections::HashMap;
use std::path::PathBuf;

#[path = "environment/runtime.rs"]
mod runtime;
#[path = "environment/rust.rs"]
mod rust;

pub use rust::rustup_home;

/// Construct the restricted environment for a sandbox invocation.
///
/// # Arguments
/// * `policy` - Controls whether temporary target paths are needed.
/// # Returns
/// Explicit environment entries, with isolated home and visible runtime paths.
/// # Examples
/// ```rust,no_run
/// let policy = codetether_sandbox_policy::SandboxPolicy::default();
/// let env = codetether_sandbox_environment::restricted(&policy);
/// assert_eq!(env["HOME"], "/tmp");
/// ```
pub fn restricted(policy: &codetether_sandbox_policy::SandboxPolicy) -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert("PATH".to_string(), search_path());
    env.insert("HOME".to_string(), "/tmp".to_string());
    env.insert("LANG".to_string(), "C.UTF-8".to_string());
    rust::extend(&mut env);
    runtime::extend(&mut env);
    if policy.allowed_paths.is_empty() {
        env.insert("TMPDIR".into(), "/tmp/.codetether/cargo-target".into());
        env.insert(
            "CARGO_TARGET_DIR".into(),
            "/tmp/.codetether/cargo-target".into(),
        );
    }
    env
}

/// Host `PATH` entries under system or exposed toolchain roots, in host
/// order, followed by the baseline system directories.
fn search_path() -> String {
    let roots = crate::toolchain::roots();
    let mut entries = crate::toolchain::path_entries(&roots);
    for baseline in ["/usr/bin", "/bin"] {
        let path = PathBuf::from(baseline);
        if !entries.contains(&path) {
            entries.push(path);
        }
    }
    std::env::join_paths(entries)
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "/usr/bin:/bin".to_string())
}
