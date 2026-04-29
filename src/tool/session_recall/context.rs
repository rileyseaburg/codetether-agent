//! Session loading and context building for recall.

use crate::session::Session;
use crate::session::helper::recall_context::messages_to_recall_context;
use anyhow::Result;

/// Load sessions and flatten into an RLM-ready string.
///
/// Uses budget-aware flattening that skips thinking blocks and truncates
/// large tool results to prevent OOM.
pub async fn build_recall_context(
    session_id: Option<String>,
    limit: usize,
) -> Result<(String, Vec<String>)> {
    let sessions = match session_id {
        Some(id) => vec![Session::load(&id).await?],
        None => load_recent_for_cwd(limit).await?,
    };

    let mut ctx = String::new();
    let mut sources = Vec::with_capacity(sessions.len());
    for s in &sessions {
        let label = format!(
            "{} ({})",
            s.title.as_deref().unwrap_or("<untitled>"),
            &s.id
        );
        sources.push(label.clone());
        ctx.push_str(&format!(
            "\n===== SESSION {label} — updated {} =====\n",
            s.updated_at
        ));
        let (flat, _truncated) = messages_to_recall_context(&s.messages);
        ctx.push_str(&flat);
    }
    Ok((ctx, sources))
}

/// Load the most recent `limit` sessions for the current working directory.
async fn load_recent_for_cwd(limit: usize) -> Result<Vec<Session>> {
    let cwd = std::env::current_dir().ok();
    let summaries = match cwd.as_deref() {
        Some(dir) => crate::session::listing::list_sessions_for_directory(dir).await?,
        None => crate::session::list_sessions().await?,
    };

    let mut loaded = Vec::new();
    for s in summaries.into_iter().take(limit) {
        match Session::load(&s.id).await {
            Ok(sess) => loaded.push(sess),
            Err(e) => tracing::warn!(session_id = %s.id, error = %e, "session_recall: load failed"),
        }
    }
    Ok(loaded)
}
