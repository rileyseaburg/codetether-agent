#[path = "sandbox_bwrap_probe_command.rs"]
mod command;

use std::path::Path;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ProbeResult {
    Usable { version: String },
    Unavailable { reason: &'static str },
}

static RESULT: OnceLock<ProbeResult> = OnceLock::new();

pub(super) fn probe(path: &Path) -> ProbeResult {
    RESULT.get_or_init(|| run(path)).clone()
}

pub(super) fn kernel_fallbacks(seccomp_active: bool, landlock_active: bool) -> Vec<String> {
    let mut out = Vec::new();
    if !seccomp_active {
        out.push("seccomp_inactive:not_configured".to_string());
    }
    if !landlock_active {
        out.push("landlock_inactive:not_configured".to_string());
    }
    out
}

pub(super) fn direct_fallbacks(reason: &str) -> Vec<String> {
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
#[path = "sandbox_bwrap_probe_tests.rs"]
mod tests;
