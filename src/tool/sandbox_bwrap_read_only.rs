//! Read-only Bubblewrap mount specifications.

use super::super::SandboxPolicy;

pub(super) fn specs(policy: &SandboxPolicy) -> Vec<(&'static str, String)> {
    policy.read_only_paths.iter()
        .map(|path| ("--ro-bind", path.display().to_string()))
        .collect()
}

#[cfg(test)]
#[path = "sandbox_bwrap_read_only_tests.rs"]
mod tests;