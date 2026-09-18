//! Decide whether the selected command needs the shared provider secrets manager.

use codetether_agent::{cli::Command, secrets};

pub(crate) async fn initialize(command: &Option<Command>, is_tui: bool, git_helper: bool) {
    if is_tui
        || git_helper
        || matches!(
            command,
            Some(Command::Clipboard(_) | Command::Windows(_) | Command::Vault(_))
        )
    {
        return;
    }
    match secrets::SecretsManager::from_env().await {
        Ok(manager) => {
            if manager.is_connected() {
                tracing::info!(source = "configured", "Vault client configured");
            }
            let _ = secrets::init_from_manager(manager);
        }
        Err(_) => {
            tracing::warn!(
                command = "codetether vault",
                "Vault is not configured; use codetether vault url and codetether vault login"
            );
        }
    }
}
