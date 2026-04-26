//! PowerShell bridge for Windows GUI operations.

use std::process::Command;

pub fn run(script: &str) -> anyhow::Result<serde_json::Value> {
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("PowerShell failed: {stderr}");
    }
    let stdout = String::from_utf8(output.stdout)?;
    Ok(serde_json::from_str(stdout.trim())?)
}

pub fn escape(value: &str) -> String {
    value.replace('`', "``").replace('\'', "''")
}
