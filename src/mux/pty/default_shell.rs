//! Platform-aware login-shell command selection.

#[cfg(unix)]
pub(in crate::mux) fn command() -> String {
    unix_command(std::env::var("SHELL").ok().as_deref())
}

#[cfg(unix)]
fn unix_command(configured: Option<&str>) -> String {
    let shell = configured.filter(|value| !value.trim().is_empty());
    let shell = shell.unwrap_or(fallback());
    format!("exec {} -l", quote(shell))
}

#[cfg(all(unix, target_os = "macos"))]
fn fallback() -> &'static str {
    "/bin/zsh"
}

#[cfg(all(unix, not(target_os = "macos")))]
fn fallback() -> &'static str {
    if std::path::Path::new("/bin/bash").is_file() {
        "/bin/bash"
    } else {
        "/bin/sh"
    }
}

#[cfg(unix)]
fn quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[cfg(windows)]
pub(in crate::mux) fn command() -> String {
    for shell in ["pwsh.exe", "powershell.exe"] {
        if which::which(shell).is_ok() {
            return format!("{shell} -NoLogo");
        }
    }
    std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".into())
}

#[cfg(not(any(unix, windows)))]
pub(in crate::mux) fn command() -> String {
    "sh".into()
}

#[cfg(test)]
#[path = "default_shell_tests.rs"]
mod tests;
