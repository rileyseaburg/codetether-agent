use std::path::PathBuf;

#[cfg(target_os = "linux")]
use super::sandbox_bwrap_probe::ProbeResult;

#[cfg(any(test, not(target_os = "linux")))]
#[path = "sandbox_runner_select_platform.rs"]
mod platform;
#[path = "sandbox_runner_select_seatbelt.rs"]
mod seatbelt;
#[path = "sandbox_bwrap_trusted.rs"]
mod trusted_bwrap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Runner {
    Bubblewrap(PathBuf),
    Seatbelt(PathBuf),
    Direct(&'static str),
}

#[cfg(target_os = "linux")]
pub(super) fn selected_runner() -> Runner {
    seatbelt::upgrade(linux_runner(), super::sandbox_seatbelt::selected())
}

#[cfg(target_os = "linux")]
fn linux_runner() -> Runner {
    let Some(path) = trusted_bwrap::find() else {
        return Runner::Direct("bwrap_not_found");
    };
    match super::sandbox_bwrap_probe::probe(&path) {
        ProbeResult::Usable { version } => {
            tracing::debug!(bwrap = %path.display(), version = %version, "bwrap sandbox available");
            Runner::Bubblewrap(path)
        }
        ProbeResult::Unavailable { reason } => Runner::Direct(reason),
    }
}

#[cfg(not(target_os = "linux"))]
pub(super) fn selected_runner() -> Runner {
    let unconfined = Runner::Direct(platform::unsupported_reason());
    seatbelt::upgrade(unconfined, super::sandbox_seatbelt::selected())
}

#[cfg(test)]
pub(super) fn select_for(is_linux: bool, bwrap: Option<PathBuf>) -> Runner {
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