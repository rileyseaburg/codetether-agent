//! Resolve the CodeTether executable used by mux-owned agent tasks.

use std::ffi::OsString;
use std::path::Path;

use anyhow::Result;

pub(super) fn command() -> Result<tokio::process::Command> {
    let current = std::env::current_exe()?;
    let executable = select(&current, std::env::var_os("CODETETHER_BIN"));
    if executable != current.as_os_str() {
        tracing::warn!(
            replaced = %current.display(),
            fallback = ?executable,
            "Mux host executable was replaced; using installed CodeTether binary"
        );
    }
    Ok(tokio::process::Command::new(executable))
}

fn select(current: &Path, configured: Option<OsString>) -> OsString {
    if current.exists() {
        return current.as_os_str().to_owned();
    }
    configured.unwrap_or_else(|| OsString::from("codetether"))
}

#[cfg(test)]
#[path = "executable_tests.rs"]
mod tests;
