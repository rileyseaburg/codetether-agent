//! Rust toolchain mounts and environment for sandboxed rust-analyzer.

use crate::tool::sandbox::SandboxPolicy;
use std::path::{Path, PathBuf};

pub(super) fn add(policy: &mut SandboxPolicy, analyzer_bin: &Path) {
    let cargo_home = home("CARGO_HOME", ".cargo");
    let rustup_home = home("RUSTUP_HOME", ".rustup");
    add_existing(policy, cargo_home.join("registry"));
    add_existing(policy, rustup_home.clone());
    add_tool(policy, "cargo", "CARGO");
    add_tool(policy, "rustc", "RUSTC");
    policy
        .environment
        .insert("CARGO_HOME".into(), cargo_home.display().to_string());
    policy
        .environment
        .insert("RUSTUP_HOME".into(), rustup_home.display().to_string());
    policy.environment.insert(
        "PATH".into(),
        format!("{}:/usr/bin:/bin", analyzer_bin.display()),
    );
}

fn add_tool(policy: &mut SandboxPolicy, command: &str, variable: &str) {
    let Ok(path) = which::which(command) else {
        return;
    };
    if let Some(parent) = path.parent() {
        let parent = parent.to_path_buf();
        if !policy.read_only_paths.contains(&parent) {
            policy.read_only_paths.push(parent);
        }
    }
    policy
        .environment
        .insert(variable.into(), path.display().to_string());
}

fn home(variable: &str, fallback: &str) -> PathBuf {
    std::env::var_os(variable)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join(fallback)
        })
}

fn add_existing(policy: &mut SandboxPolicy, path: PathBuf) {
    if path.exists() {
        policy.read_only_paths.push(path);
    }
}
