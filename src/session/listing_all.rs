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

    let codex = match list_codex_sessions_for_directory(dir) {
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
        match merged.get(&session.id) {
            Some(existing) if existing.updated_at >= session.updated_at => {}
            _ => {
                merged.insert(session.id.clone(), session);
            }
        }
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
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn test_list_sessions_real_directory() {
        let dir = Path::new("/home/riley/A2A-Server-MCP/codetether-agent");
        let result = list_all_sessions_for_directory(dir).await;
        match &result {
            Ok(sessions) => eprintln!("Found {} sessions", sessions.len()),
            Err(err) => eprintln!("Error: {err}"),
        }
        assert!(
            result.is_ok(),
            "list_all_sessions_for_directory should not error"
        );
    }

    #[test]
    fn test_list_codex_sessions_real_directory() {
        let dir = Path::new("/home/riley/A2A-Server-MCP/codetether-agent");
        let result = list_codex_sessions_for_directory(dir);
        match &result {
            Ok(sessions) => eprintln!("Found {} codex sessions", sessions.len()),
            Err(err) => eprintln!("Codex error: {err}"),
        }
        assert!(
            result.is_ok(),
            "list_codex_sessions_for_directory should not error"
        );
    }
}
