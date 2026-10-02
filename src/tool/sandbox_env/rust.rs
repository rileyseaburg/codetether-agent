//! Locate readable Rustup metadata without exposing the host home or Cargo secrets.

use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn rustup_home() -> Option<PathBuf> {
    std::env::var_os("RUSTUP_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".rustup")))
        .filter(|path| path.is_absolute() && path.is_dir())
}

pub(super) fn extend(env: &mut HashMap<String, String>) {
    if let Some(home) = rustup_home() {
        env.insert("RUSTUP_HOME".into(), home.to_string_lossy().into_owned());
    }
    // Do not inherit host Cargo credentials. Cache writes still require a
    // permitted writable path (a temporary mount when available, or an explicit
    // workspace CARGO_HOME); Rustup metadata remains read-only.
    env.insert("CARGO_HOME".into(), "/tmp/.cargo".into());
}

#[cfg(test)]
#[path = "rust_tests.rs"]
mod tests;
