//! Build a Seatbelt (SBPL) profile from a sandbox policy.

use super::super::SandboxPolicy;
use super::base::{ALLOW_NETWORK, HEADER, WRITABLE_DEVICES};
use super::paths::{protected_paths, writable_roots};
use super::quote::quote;
use std::path::Path;

/// Render the SBPL profile text that confines one sandboxed command.
///
/// Rule order matters: writable roots are granted before protected subpaths
/// are denied, because Seatbelt applies the last matching rule.
pub(super) fn build(policy: &SandboxPolicy, work_dir: &Path, temp_dir: &Path) -> String {
    let mut lines: Vec<String> = HEADER.iter().map(|rule| (*rule).to_string()).collect();
    lines.push(device_writes());
    if policy.allow_network {
        lines.push(ALLOW_NETWORK.to_string());
    }
    for root in writable_roots(policy, work_dir, temp_dir) {
        lines.push(write_rule("allow", &root));
    }
    for denied in protected_paths(policy) {
        lines.push(write_rule("deny", &denied));
    }
    lines.join("\n")
}

fn device_writes() -> String {
    let literals = WRITABLE_DEVICES
        .iter()
        .map(|path| format!("(literal {})", quote(path)))
        .collect::<Vec<_>>()
        .join(" ");
    format!("(allow file-write-data {literals})")
}

fn write_rule(effect: &str, path: &str) -> String {
    format!("({effect} file-write* (subpath {}))", quote(path))
}

#[cfg(test)]
#[path = "sandbox_seatbelt_profile_tests.rs"]
mod tests;
