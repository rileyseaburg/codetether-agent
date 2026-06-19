//! Background task spawning for TUI forage runs.

use crate::cli::ForageArgs;

use super::state::ForageUpdate;

/// Spawn a background task that runs forage to completion and streams
/// the final summary (or error) back through `tx`.
///
/// When the run was a scan (not execute) and found opportunities, emits an
/// [`ForageUpdate::Offer`] instead of a plain `Complete` so the TUI can ask
/// the user "want me to start?" in plain English.
pub fn spawn_forage_run(args: ForageArgs, tx: tokio::sync::mpsc::Sender<ForageUpdate>) {
    // Suppress forage's direct stdout writes; the TUI renders updates via `tx`
    // and raw `println!` would corrupt the ratatui screen.
    crate::forage::console::set_quiet(true);
    let scan_only = !args.execute;
    let top = args.top;
    let model = args.model.clone();
    tokio::spawn(async move {
        let _ = tx
            .send(ForageUpdate::Status(
                "Scanning OKRs for opportunities…".to_string(),
            ))
            .await;
        match crate::forage::execute_with_summary(args).await {
            Ok(summary) => {
                let selected = summary.total_selected;
                if scan_only && selected > 0 {
                    let _ = tx
                        .send(ForageUpdate::Offer {
                            text: summary.render_text(),
                            selected,
                            top,
                            model,
                        })
                        .await;
                } else {
                    let _ = tx.send(ForageUpdate::Complete(summary.render_text())).await;
                }
            }
            Err(err) => {
                let _ = tx.send(ForageUpdate::Error(err.to_string())).await;
            }
        }
    });
}
