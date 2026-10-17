//! Probe the configured Bun executable.
use super::*;
use tokio::process::Command;
impl ExternalRepl {
    pub(super) async fn is_bun_available() -> bool {
        Command::new("bun")
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}
