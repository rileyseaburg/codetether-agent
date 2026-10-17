//! runner select for sandbox execution planning.
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use super::sandbox_bwrap_probe::ProbeResult;

#[cfg(any(test, not(target_os = "linux")))]
#[path = "sandbox_runner_select_platform.rs"]
mod platform;
#[path = "sandbox_runner_select_seatbelt.rs"]
mod seatbelt;

/// Selected launch backend: Bubblewrap, Seatbelt, or direct execution with a reason.
///
/// # Examples
/// ```rust
/// let runner = codetether_sandbox_runner::Runner::Direct("not_available");
/// assert!(matches!(runner, codetether_sandbox_runner::Runner::Direct(_)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Runner {
    Bubblewrap(PathBuf),
    Seatbelt(PathBuf),
    Direct(&'static str),
}

#[cfg(target_os = "linux")]
/// Select the current platform backend using the cached capability probe.
///
/// # Returns
/// The selected backend or a reason that OS confinement is unavailable.
pub fn selected_runner() -> Runner {
    seatbelt::upgrade(linux_runner(), super::sandbox_seatbelt::selected())
}

#[cfg(target_os = "linux")]
fn linux_runner() -> Runner {
    let Ok(path) = which::which("bwrap") else {
        return Runner::Direct("bwrap_not_found");
    };
    match super::sandbox_bwrap_probe::probe(&path) {
        ProbeResult::Usable { version } => {
            tracing::debug!(target: "codetether_agent::tool::sandbox::sandbox_runner_select",bwrap = %path.display(), version = %version, "bwrap sandbox available");
            Runner::Bubblewrap(path)
        }
        ProbeResult::Unavailable { reason } => Runner::Direct(reason),
    }
}

#[cfg(not(target_os = "linux"))]
pub fn selected_runner() -> Runner {
    let unconfined = Runner::Direct(platform::unsupported_reason());
    seatbelt::upgrade(unconfined, super::sandbox_seatbelt::selected())
}

#[cfg(test)]
pub fn select_for(is_linux: bool, bwrap: Option<PathBuf>) -> Runner {
    if !is_linux {
        return Runner::Direct(platform::unsupported_reason());
    }
    bwrap
        .map(Runner::Bubblewrap)
        .unwrap_or(Runner::Direct("bwrap_not_found"))
}

#[cfg(test)]
#[path = "sandbox_runner_select_tests.rs"]
mod tests;
