//! Execute a configured server installation and report command failures.

use super::*;

pub(super) async fn run_install(config: &LspConfig) -> Result<()> {
    let Some(install_args) = install_command_for(&config.command) else {
        return Err(anyhow::anyhow!(
            "Language server '{}' not found and no auto-install available. Install it manually.",
            config.command,
        ));
    };

    info!(command = %config.command, "Language server not found, installing...");

    let output = tokio::process::Command::new(install_args[0])
        .args(&install_args[1..])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return Err(anyhow::anyhow!(
            "Failed to install '{}' (exit code {:?}). stdout: {} stderr: {}",
            config.command,
            output.status.code(),
            stdout,
            stderr,
        ));
    }

    Ok(())
}
