//! Windows bash shell discovery helpers.

use std::path::PathBuf;

pub(super) fn git_bash_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for env_var in ["ProgramFiles", "ProgramFiles(x86)", "ProgramW6432"] {
        if let Ok(base) = std::env::var(env_var) {
            out.push(PathBuf::from(base).join("Git").join("bin").join("bash.exe"));
        }
    }
    if let Ok(base) = std::env::var("LOCALAPPDATA") {
        out.push(
            PathBuf::from(base)
                .join("Programs")
                .join("Git")
                .join("bin")
                .join("bash.exe"),
        );
    }
    out
}
