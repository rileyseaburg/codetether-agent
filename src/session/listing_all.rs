use super::codex_import::discover_codex_sessions_for_directory;
use super::listing::{SessionSummary, list_sessions_for_directory, merge_summary};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub fn list_codex_sessions_for_directory(dir: &Path) -> Result<Vec<SessionSummary>> {
    Ok(discover_codex_sessions_for_directory(dir)?
        .into_iter()
        .map(|session| SessionSummary {
            id: session.id.clone(),
            title: Some(format!("[Codex] {}", session.title.unwrap_or(session.id))),
            created_at: session.created_at,
            updated_at: session.updated_at,
            message_count: session.message_count,
            agent: session.agent,
            directory: session.directory,
        })
        .collect())
}

pub async fn list_all_sessions_for_directory(dir: &Path) -> Result<Vec<SessionSummary>> {
    tracing::info!(dir = %dir.display(), "list_all_sessions_for_directory: starting");
    let mut merged = HashMap::<String, SessionSummary>::new();

    let native = list_sessions_for_directory(dir).await?;
    tracing::info!(
        native_count = native.len(),
        "list_all_sessions_for_directory: native sessions"
    );

    for session in native {
        merged.insert(session.id.clone(), session);
    }

    // Codex discovery walks and reads every archive file synchronously; run it
    // off the async executor so a TUI event loop awaiting this stays responsive.
    let dir_owned = dir.to_path_buf();
    let codex = tokio::task::spawn_blocking(move || list_codex_sessions_for_directory(&dir_owned))
        .await
        .map_err(anyhow::Error::from)
        .and_then(|result| result);
    let codex = match codex {
        Ok(c) => {
            tracing::info!(
                codex_count = c.len(),
                "list_all_sessions_for_directory: codex sessions"
            );
            c
        }
        Err(err) => {
            tracing::warn!(error = %err, "list_all_sessions_for_directory: codex discovery failed");
            Vec::new()
        }
    };

    for session in codex {
        let summary = match merged.remove(&session.id) {
            Some(native) => merge_summary(native, session),
            None => session,
        };
        merged.insert(summary.id.clone(), summary);
    }

    let mut sessions = merged.into_values().collect::<Vec<_>>();
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    tracing::info!(
        total = sessions.len(),
        "list_all_sessions_for_directory: done"
    );
    Ok(sessions)
}

#[cfg(test)]
#[path = "listing_all_tests.rs"]
mod tests;
