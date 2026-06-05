//! Cross-platform shell resolution for the `bash` tool.
//!
//! On Unix we just invoke `/bin/bash -c <cmd>`. On Windows we probe a ranked
//! list of bash-compatible shells so the tool keeps working even on hosts
//! that don't have `bash.exe` in `PATH`:
//!
//! 1. `bash` (e.g. Git for Windows installed and on `PATH`)
//! 2. `C:\Program Files\Git\bin\bash.exe` (standard Git for Windows install)
//! 3. `C:\Program Files (x86)\Git\bin\bash.exe` (32-bit Git install)
//! 4. `%LOCALAPPDATA%\Programs\Git\bin\bash.exe` (user-scoped Git install)
//! 5. `wsl.exe -- bash -c <cmd>` (any WSL distro)
//!
//! If none are available we fall back to `cmd.exe /C <cmd>` — not bash, but
//! at least the tool runs *something* rather than silently failing. Commands
//! that rely on bash-isms will still error, but the failure is visible.
//!
//! The result is a `(program, argv_prefix)` pair: the caller appends the
//! command string and spawns normally through `tokio::process::Command`.

/// Resolved shell invocation: `(program, leading args before the command)`.
///
/// Example on Unix: `("bash".into(), vec!["-c".into()])`.
/// Example on Windows with WSL only: `("wsl.exe".into(), vec!["--".into(), "bash".into(), "-c".into()])`.
pub(super) struct Shell {
    pub program: String,
    pub prefix_args: Vec<String>,
}

#[cfg(not(target_os = "windows"))]
pub(super) fn resolve() -> Shell {
    let program = if std::path::Path::new("/bin/bash").is_file() {
        "/bin/bash"
    } else {
        "bash"
    };
    Shell {
        program: program.into(),
        prefix_args: vec!["-c".into()],
    }
}

#[cfg(target_os = "windows")]
pub(super) fn resolve() -> Shell {
    if which::which("bash").is_ok() {
        return Shell {
            program: "bash".into(),
            prefix_args: vec!["-c".into()],
        };
    }
    for path in git_bash_candidates() {
        if path.is_file() {
            return Shell {
                program: path.to_string_lossy().into_owned(),
                prefix_args: vec!["-c".into()],
            };
        }
    }
    if which::which("wsl.exe").is_ok() {
        return Shell {
            program: "wsl.exe".into(),
            prefix_args: vec!["--".into(), "bash".into(), "-c".into()],
        };
    }
    // Last resort: cmd.exe. Note: bash-syntax commands will fail here, but
    // simple one-liners (`echo`, `dir`, `where`) still work — better than a
    // hard "no shell" abort.
    Shell {
        program: "cmd.exe".into(),
        prefix_args: vec!["/C".into()],
    }
}

#[cfg(target_os = "windows")]
#[path = "bash_shell_windows.rs"]
mod windows;
#[cfg(target_os = "windows")]
use windows::git_bash_candidates;
