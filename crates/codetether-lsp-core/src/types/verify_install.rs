//! Report whether a newly installed server is discoverable.

use super::*;

pub(super) async fn verify_install(config: &LspConfig) {
    // Verify installation succeeded
    if which::which(&config.command).is_err() {
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
                info!(command = %config.command, "Language server installed and available via rustup run stable");
                return;
            }
        }
        warn!(command = %config.command, "Install succeeded but binary still not found on PATH");
    } else {
        info!(command = %config.command, "Language server installed successfully");
    }
}
