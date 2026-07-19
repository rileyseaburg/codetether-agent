//! Legacy-session recall sidecar backfill worker.

use std::path::PathBuf;

const MAX_MIGRATIONS_PER_RUN: usize = 8;

pub(super) async fn run(workspace: PathBuf) {
    let summaries = match crate::session::listing::list_sessions_for_directory(&workspace).await {
        Ok(summaries) => summaries,
        Err(error) => {
            tracing::warn!(%error, "recall backfill session listing failed");
            return;
        }
    };
    let mut migrated = 0;
    for summary in summaries {
        if migrated >= MAX_MIGRATIONS_PER_RUN {
            break;
        }
        if super::session_io::read(&summary.id).await.is_some() {
            continue;
        }
        let Some(session) = super::backfill_load::bounded(&summary.id).await else {
            continue;
        };
        let Some(indexed) = tokio::task::spawn_blocking(move || super::build::session(&session))
            .await
            .ok()
            .flatten()
        else {
            continue;
        };
        if let Err(error) = super::store::upsert(indexed).await {
            tracing::warn!(session_id = %summary.id, %error, "recall backfill failed");
        } else {
            migrated += 1;
        }
    }
}
