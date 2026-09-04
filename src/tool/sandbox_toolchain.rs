//! Host toolchains exposed read-only inside the command sandbox.
//!
//! The sandbox resets `PATH` and mounts only system roots, so user-installed
//! runtimes (nvm Node, pnpm, cargo, bun) are invisible: commands fail with
//! `not found` or silently fall back to an older system binary. Well-known
//! toolchain roots under `$HOME`, plus any listed in
//! `CODETETHER_SANDBOX_TOOLCHAIN_PATHS`, are exposed read-only and the host
//! `PATH` entries inside them are kept in host order.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[path = "sandbox_toolchain_defaults.rs"]
mod defaults;

/// Colon-separated extra toolchain roots to expose read-only.
pub(super) const ENV: &str = "CODETETHER_SANDBOX_TOOLCHAIN_PATHS";
const SYSTEM: &[&str] = &["/usr", "/bin", "/sbin", "/lib", "/lib64"];

/// Existing toolchain roots from the override env var and known defaults.
pub(super) fn roots() -> Vec<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    roots_from(std::env::var_os(ENV).as_deref(), home.as_deref())
}

pub(super) fn roots_from(configured: Option<&OsStr>, home: Option<&Path>) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = configured
        .map(std::env::split_paths)
        .into_iter()
        .flatten()
        .collect();
    if let Some(home) = home {
        roots.extend(defaults::RELATIVE.iter().map(|relative| home.join(relative)));
    }
    let mut unique = Vec::new();
    for root in roots {
        if root.is_absolute() && root.is_dir() && !unique.contains(&root) {
            unique.push(root);
        }
    }
    unique
}

/// Host `PATH` entries that resolve inside a system or toolchain root.
pub(super) fn path_entries(roots: &[PathBuf]) -> Vec<PathBuf> {
    path_entries_from(&std::env::var_os("PATH").unwrap_or_default(), roots)
}

pub(super) fn path_entries_from(host: &OsStr, roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut entries = Vec::new();
    for entry in std::env::split_paths(host) {
        let visible = SYSTEM.iter().any(|system| entry.starts_with(system))
            || roots.iter().any(|root| entry.starts_with(root));
        if visible && !entries.contains(&entry) {
            entries.push(entry);
        }
    }
    entries
}

#[cfg(test)]
#[path = "sandbox_toolchain_live_tests.rs"]
mod live_tests;
#[cfg(test)]
#[path = "sandbox_toolchain_tests.rs"]
mod tests;
