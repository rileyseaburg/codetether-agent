//! Background task spawning for TUI forage runs.

use crate::cli::ForageArgs;

use super::state::ForageUpdate;

/// Spawn a background task that runs forage to completion and streams
/// the final summary (or error) back through `tx`.
///
/// Scan-only and execution runs emit distinct completion updates so the TUI
/// can clearly state whether any work was executed.
pub fn spawn_forage_run(args: ForageArgs, tx: tokio::sync::mpsc::Sender<ForageUpdate>) {
    // Suppress forage's direct stdout writes; the TUI renders updates via `tx`
    // and raw `println!` would corrupt the ratatui screen.
    crate::forage::console::set_quiet(true);
    let scan_only = !args.execute;
    tokio::spawn(async move {
        let _ = tx
            .send(ForageUpdate::Status(
                "Scanning OKRs for opportunities…".to_string(),
            ))
            .await;
        match crate::forage::execute_with_summary(args).await {
            Ok(summary) => {
                let text = summary.render_text();
                if scan_only {
                    let _ = tx.send(ForageUpdate::ScanComplete(text)).await;
                } else {
                    let _ = tx.send(ForageUpdate::Complete(text)).await;
                }
            }
            Err(err) => {
                let _ = tx.send(ForageUpdate::Error(err.to_string())).await;
            }
        }
    });
}
