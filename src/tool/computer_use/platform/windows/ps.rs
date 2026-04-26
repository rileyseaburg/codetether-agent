//! PowerShell bridge for Windows GUI operations.

use tokio::process::Command;
use tokio::time::{Duration, timeout};

pub async fn run(script: &str) -> anyhow::Result<serde_json::Value> {
    let output = timeout(
        Duration::from_secs(30),
        Command::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command", script])
            .kill_on_drop(true)
            .output(),
    )
    .await
    .map_err(|_| anyhow::anyhow!("PowerShell timed out"))??;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("PowerShell failed: {stderr}");
    }
    parse_json_line(&String::from_utf8(output.stdout)?)
}

pub fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

fn parse_json_line(stdout: &str) -> anyhow::Result<serde_json::Value> {
    let line = stdout
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("PowerShell produced no JSON output"))?;
    Ok(serde_json::from_str(line.trim())?)
}
