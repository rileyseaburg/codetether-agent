use super::codex_import::discover_codex_sessions_for_directory;
use super::listing::{SessionSummary, list_sessions_for_directory};
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
    let mut merged = HashMap::<String, SessionSummary>::new();

    for session in list_sessions_for_directory(dir).await? {
        merged.insert(session.id.clone(), session);
    }

    for session in list_codex_sessions_for_directory(dir)? {
        match merged.get(&session.id) {
            Some(existing) if existing.updated_at >= session.updated_at => {}
            _ => {
                merged.insert(session.id.clone(), session);
            }
        }
    }

    let mut sessions = merged.into_values().collect::<Vec<_>>();
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions)
}
