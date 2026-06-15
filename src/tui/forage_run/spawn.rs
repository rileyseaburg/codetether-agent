//! Background task spawning for TUI forage runs.

use crate::cli::ForageArgs;

use super::state::ForageUpdate;

/// Spawn a background task that runs forage to completion and streams
/// the final summary (or error) back through `tx`.
pub fn spawn_forage_run(args: ForageArgs, tx: tokio::sync::mpsc::Sender<ForageUpdate>) {
    tokio::spawn(async move {
        let _ = tx
            .send(ForageUpdate::Status(
                "Scanning OKRs for opportunities…".to_string(),
            ))
            .await;
        match crate::forage::execute_with_summary(args).await {
            Ok(summary) => {
                let _ = tx.send(ForageUpdate::Complete(summary.render_text())).await;
            }
            Err(err) => {
                let _ = tx.send(ForageUpdate::Error(err.to_string())).await;
            }
        }
    });
}
