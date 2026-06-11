use std::path::PathBuf;

use super::sandbox_bwrap_probe::ProbeResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Runner {
    Bubblewrap(PathBuf),
    Direct(&'static str),
}

#[cfg(target_os = "linux")]
pub(super) fn selected_runner() -> Runner {
    let Ok(path) = which::which("bwrap") else {
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
    select_for(false, None)
}

pub(super) fn select_for(is_linux: bool, bwrap: Option<PathBuf>) -> Runner {
    if !is_linux {
        return Runner::Direct("non_linux");
    }
    bwrap
        .map(Runner::Bubblewrap)
        .unwrap_or(Runner::Direct("bwrap_not_found"))
}

#[cfg(test)]
#[path = "sandbox_runner_select_tests.rs"]
mod tests;
