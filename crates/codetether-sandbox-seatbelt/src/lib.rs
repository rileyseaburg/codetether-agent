//! macOS Seatbelt profile generation and staging.
//!
//! Profile text generation is platform independent so it can be unit tested on
//! any host; only [`selected`] consults the running OS.

use anyhow::Result;
use codetether_sandbox_policy::SandboxPolicy;
use std::path::{Path, PathBuf};

#[path = "base.rs"]
mod base;
#[path = "paths.rs"]
mod paths;
#[path = "profile.rs"]
mod profile_text;
#[path = "quote.rs"]
mod quote;
#[path = "roots.rs"]
mod roots;
#[path = "stage.rs"]
mod stage;

/// Render the SBPL profile confining one command.
///
/// # Arguments
/// * `policy` - Allowed writes and networking.
/// * `work_dir` - Command working directory.
/// * `temp_dir` - Temporary writable directory.
/// # Returns
/// Seatbelt profile text with protected-path denies after writable grants.
/// # Examples
/// ```rust
/// let policy = codetether_sandbox_policy::SandboxPolicy::default();
/// let text = codetether_sandbox_seatbelt::profile(
///     &policy, std::path::Path::new("/workspace"), std::path::Path::new("/tmp"),
/// );
/// assert!(text.starts_with("(version 1)"));
/// ```
pub fn profile(policy: &SandboxPolicy, work_dir: &Path, temp_dir: &Path) -> String {
    profile_text::build(policy, work_dir, temp_dir)
}

/// Write `profile` to a uniquely named file and return its path.
///
/// # Errors
///
/// Returns an error when the profile file cannot be created or written.
///
/// # Arguments
/// * `profile` - Rendered SBPL text.
/// * `temp_dir` - Directory for the uniquely named profile file.
/// # Returns
/// Path to the staged profile.
/// # Examples
/// ```rust,no_run
/// let _path = codetether_sandbox_seatbelt::write_profile(
///     "(version 1)", std::path::Path::new("/tmp"),
/// )?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn write_profile(profile: &str, temp_dir: &Path) -> Result<PathBuf> {
    stage::write(profile, temp_dir)
}

/// Isolation gaps that Seatbelt cannot close, reported to callers.
///
/// # Returns
/// Isolation-gap labels for the supplied policy.
/// # Examples
/// ```rust
/// let gaps = codetether_sandbox_seatbelt::gaps(&Default::default());
/// assert!(gaps.contains(&"seatbelt_read_unconfined".to_string()));
/// ```
pub fn gaps(policy: &SandboxPolicy) -> Vec<String> {
    let mut out = vec!["seatbelt_read_unconfined".to_string()];
    if policy.allow_network {
        out.push("seatbelt_network_allowed".to_string());
    }
    out
}

/// Absolute path to `sandbox-exec` when this host can enforce Seatbelt.
///
/// # Returns
/// The executable path on a supporting macOS host, otherwise `None`.
/// # Examples
/// ```rust,no_run
/// let _executable = codetether_sandbox_seatbelt::selected();
/// ```
pub fn selected() -> Option<PathBuf> {
    let path = PathBuf::from("/usr/bin/sandbox-exec");
    cfg!(target_os = "macos")
        .then_some(path)
        .filter(|p| p.exists())
}
