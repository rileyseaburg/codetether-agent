//! Minimal sandbox environment with explicitly exposed Rust toolchain homes.

use std::collections::HashMap;
use std::path::PathBuf;

#[path = "sandbox_env/runtime.rs"]
mod runtime;
#[path = "sandbox_env/rust.rs"]
mod rust;

pub(super) use rust::rustup_home;

pub(super) fn restricted() -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert("PATH".to_string(), search_path());
    env.insert("HOME".to_string(), "/tmp".to_string());
    env.insert("LANG".to_string(), "C.UTF-8".to_string());
    rust::extend(&mut env);
    runtime::extend(&mut env);
    env
}

/// Host `PATH` entries under system or exposed toolchain roots, in host
/// order, followed by the baseline system directories.
fn search_path() -> String {
    let roots = super::sandbox_toolchain::roots();
    let mut entries = super::sandbox_toolchain::path_entries(&roots);
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
