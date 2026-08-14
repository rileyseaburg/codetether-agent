//! macOS Seatbelt profile generation and staging.
//!
//! Profile text generation is platform independent so it can be unit tested on
//! any host; only [`selected`] consults the running OS.

use super::SandboxPolicy;
use anyhow::Result;
use std::path::{Path, PathBuf};

#[path = "sandbox_seatbelt_base.rs"]
mod base;
#[path = "sandbox_seatbelt_paths.rs"]
mod paths;
#[path = "sandbox_seatbelt_profile.rs"]
mod profile_text;
#[path = "sandbox_seatbelt_quote.rs"]
mod quote;
#[path = "sandbox_seatbelt_roots.rs"]
mod roots;
#[path = "sandbox_seatbelt_stage.rs"]
mod stage;

/// Render the SBPL profile confining one command.
pub(super) fn profile(policy: &SandboxPolicy, work_dir: &Path, temp_dir: &Path) -> String {
    profile_text::build(policy, work_dir, temp_dir)
}

/// Write `profile` to a uniquely named file and return its path.
///
/// # Errors
///
/// Returns an error when the profile file cannot be created or written.
pub(super) fn write_profile(profile: &str, temp_dir: &Path) -> Result<PathBuf> {
    stage::write(profile, temp_dir)
}

/// Isolation gaps that Seatbelt cannot close, reported to callers.
pub(super) fn gaps(policy: &SandboxPolicy) -> Vec<String> {
    let mut out = vec!["seatbelt_read_unconfined".to_string()];
    if policy.allow_network {
        out.push("seatbelt_network_allowed".to_string());
    }
    out
}

/// Absolute path to `sandbox-exec` when this host can enforce Seatbelt.
pub(super) fn selected() -> Option<PathBuf> {
    let path = PathBuf::from("/usr/bin/sandbox-exec");
    cfg!(target_os = "macos")
        .then_some(path)
        .filter(|p| p.exists())
}
