use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Supported explicit operations; no global enable/install operation exists.
///
/// ```
/// use codetether_rust_build::RustAction;
/// let action = RustAction::Check;
/// assert!(matches!(action, RustAction::Check));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RustAction {
    /// Build with ra-jit.
    #[default]
    Build,
    /// Build and run with ra-jit.
    Run,
    /// Execute tests with ra-jit (not authoritative rustc validation).
    Test,
    /// Normal Cargo checking, including borrow checking.
    Check,
    /// Build with the repository release pipeline, then attempt guarded publication.
    #[serde(rename = "cache-build")]
    CacheBuild,
    /// Publish verified release artifacts through repository guards, without rebuilding.
    #[serde(rename = "cache-publish")]
    CachePublish,
    /// Show ra-jit's per-user daemon status.
    Status,
    /// Stop the per-user daemon, affecting other clients too.
    Stop,
    /// Report prerequisites without installation/configuration changes.
    Doctor,
}
