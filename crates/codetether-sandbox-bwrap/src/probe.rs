//! probe for bubblewrap capability detection.
#[path = "probe_command.rs"]
mod command;

use std::path::Path;
use std::sync::OnceLock;

/// Cached result of checking bubblewrap's version and minimal invocation.
///
/// # Examples
/// ```rust
/// use codetether_sandbox_bwrap::ProbeResult;
/// let result = ProbeResult::Unavailable { reason: "bwrap_smoke_failed" };
/// assert!(matches!(result, ProbeResult::Unavailable { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeResult {
    Usable { version: String },
    Unavailable { reason: &'static str },
}

static RESULT: OnceLock<ProbeResult> = OnceLock::new();

/// Probe once and return the process-wide cached capability result.
///
/// # Arguments
/// * `path` - Bubblewrap executable checked on the first call.
/// # Returns
/// Version and usability, or a stable unavailability reason.
/// # Examples
/// ```rust,no_run
/// let _result = codetether_sandbox_bwrap::probe(std::path::Path::new("/usr/bin/bwrap"));
/// ```
pub fn probe(path: &Path) -> ProbeResult {
    RESULT.get_or_init(|| run(path)).clone()
}

/// Report missing seccomp or Landlock layers in their original order.
///
/// # Returns
/// Zero or more isolation-gap labels.
/// # Examples
/// ```rust
/// assert!(codetether_sandbox_bwrap::kernel_fallbacks(true, true).is_empty());
/// ```
pub fn kernel_fallbacks(seccomp_active: bool, landlock_active: bool) -> Vec<String> {
    let mut out = Vec::new();
    if !seccomp_active {
        out.push("seccomp_inactive:not_configured".to_string());
    }
    if !landlock_active {
        out.push("landlock_inactive:not_configured".to_string());
    }
    out
}

/// Append the OS-sandbox failure reason after both kernel-layer gaps.
///
/// # Returns
/// Labels explaining direct execution's isolation gaps.
/// # Examples
/// ```rust
/// assert_eq!(codetether_sandbox_bwrap::direct_fallbacks("missing").len(), 3);
/// ```
pub fn direct_fallbacks(reason: &str) -> Vec<String> {
    let mut out = kernel_fallbacks(false, false);
    out.push(format!("os_sandbox_unavailable:{reason}"));
    out
}

fn run(path: &Path) -> ProbeResult {
    let Some(version) = command::version(path) else {
        return unavailable("bwrap_version_unavailable");
    };
    if command::smoke(path) {
        ProbeResult::Usable { version }
    } else {
        unavailable("bwrap_smoke_failed")
    }
}

fn unavailable(reason: &'static str) -> ProbeResult {
    ProbeResult::Unavailable { reason }
}

#[cfg(test)]
#[path = "probe_tests.rs"]
mod tests;
