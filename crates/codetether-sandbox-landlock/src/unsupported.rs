//! unsupported for Landlock confinement.
use codetether_sandbox_policy::SandboxPolicy;
use std::path::Path;

#[derive(Debug, Clone)]
/// Owned path rules to install when the child is spawned.
pub struct Rules;

#[derive(Debug)]
/// Rule preparation outcome and optional confinement fallback reason.
pub struct Prepared {
    pub rules: Option<Rules>,
    pub fallback: Option<String>,
}

/// Build path rules and report a fallback reason when unavailable.
pub fn prepare(_policy: &SandboxPolicy, _work_dir: &Path) -> Prepared {
    Prepared {
        rules: None,
        fallback: Some("landlock_inactive:unsupported_platform".to_string()),
    }
}

/// Register pre-exec confinement for the supplied rules.
/// No restrictions are applied to the calling process.
pub fn apply(_cmd: &mut tokio::process::Command, _rules: Option<Rules>) {}

pub fn abi_version() -> i64 {
    0
}
