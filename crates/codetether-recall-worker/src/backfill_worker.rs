//! Legacy-session recall sidecar backfill worker.

use std::path::PathBuf;

const MAX_MIGRATIONS_PER_RUN: usize = 8;

pub(super) async fn run<H: crate::Host>(workspace: PathBuf, host: &H) {
    let summaries = match host.list_sessions(&workspace).await {
        Ok(summaries) => summaries,
        Err(error) => {
            tracing::warn!(target: "codetether_agent::session::recall::backfill_worker",%error, "recall backfill session listing failed");
            return;
        }
    };
    let mut migrated = 0;
    for summary in summaries {
        if migrated >= MAX_MIGRATIONS_PER_RUN {
            break;
        }
        if codetether_recall_store::read(&summary, host)
            .await
            .is_some()
        {
            continue;
        }
        let Some(session) = super::backfill_load::bounded(&summary, host).await else {
            continue;
        };
        let Some(indexed) = tokio::task::spawn_blocking(move || H::build(&session))
            .await
            .ok()
            .flatten()
        else {
            continue;
        };
        if let Err(error) = codetether_recall_store::upsert(indexed, host).await {
            tracing::warn!(target: "codetether_agent::session::recall::backfill_worker",session_id = %summary, %error, "recall backfill failed");
        } else {
            migrated += 1;
        }
    }
}
