//! LSP install definitions and helpers.

use super::*;

/// Ensure a language server binary is available, installing it if possible.
pub async fn ensure_server_installed(config: &LspConfig) -> Result<()> {
    // Check if the binary is already on PATH.
    if which::which(&config.command).is_ok() {
        return Ok(());
    }

    // rust-analyzer is commonly installed via rustup but may not be visible on PATH
    // in the current process environment. Fall back to `rustup run <toolchain> rust-analyzer`.
    if config.command == "rust-analyzer" {
        let rustup_status = tokio::process::Command::new("rustup")
            .args(["run", "stable", "rust-analyzer", "--version"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await;

        if let Ok(status) = rustup_status
            && status.success()
        {
            return Ok(());
        }
    }

    run_install(config).await?;
    verify_install(config).await;
    Ok(())
}
