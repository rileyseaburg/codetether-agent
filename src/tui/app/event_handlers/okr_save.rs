//! Helper to persist OKR decisions in the background.
//!
//! Spawns a Tokio task that writes the OKR and its run to the
//! repository, logging any failures.
//!
//! # Examples
//!
//! ```ignore
//! spawn_okr_save(Arc::clone(&repo), okr, run);
//! ```

use std::sync::Arc;

/// Spawn a background task that persists an OKR and its run.
///
/// Logs errors if the repository write fails but does not
/// propagate them to the caller.
///
/// # Examples
///
/// ```ignore
/// spawn_okr_save(Arc::clone(&repo), okr, run);
/// ```
#[allow(dead_code)]
pub(super) fn spawn_okr_save(
    repo: Arc<crate::okr::OkrRepository>,
    okr: crate::okr::Okr,
    run: crate::okr::OkrRun,
) {
    tokio::spawn(async move {
        if let Err(e) = repo.create_okr(okr).await {
            tracing::error!(error = %e, "Failed to save approved OKR");
        }
        if let Err(e) = repo.create_run(run).await {
            tracing::error!(error = %e, "Failed to save approved OKR run");
        }
    });
}
