//! Helper to persist OKR decisions in the background.
//!
//! Spawns a Tokio task that writes the OKR and its run to the
//! repository, logging any failures.
//!
//! # Examples
//!
//! ```text
//! spawn_okr_save(repo, okr, run);
//! ```

use std::sync::Arc;

/// Spawn a background task that persists an OKR and its run.
///
/// Logs errors if the repository write fails but does not
/// propagate them to the caller.
///
/// # Examples
///
/// ```text
/// spawn_okr_save(repo, okr, run);
/// ```
#[allow(dead_code)]
pub fn spawn_okr_save(
    repo: Arc<codetether_okr::OkrRepository>,
    okr: codetether_okr::Okr,
    run: codetether_okr::OkrRun,
) {
    tokio::spawn(async move {
        if let Err(e) = repo.create_okr(okr).await {
            tracing::error!(target: "codetether_agent::tui::app::event_handlers::okr_save", error = %e, "Failed to save approved OKR");
        }
        if let Err(e) = repo.create_run(run).await {
            tracing::error!(target: "codetether_agent::tui::app::event_handlers::okr_save", error = %e, "Failed to save approved OKR run");
        }
    });
}
