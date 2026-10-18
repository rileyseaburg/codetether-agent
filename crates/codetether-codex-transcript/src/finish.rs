//! Finalize transcript fields after the archive has been consumed.
use crate::{parsed::ParsedSession, state::State};
use anyhow::{Context, Result};
use codetether_codex_discovery::derive_title_from_text;
use std::path::{Path, PathBuf};
pub(crate) fn finish(
    state: State,
    path: &Path,
    title_override: Option<&str>,
) -> Result<ParsedSession> {
    let meta = state
        .session_meta
        .with_context(|| format!("Missing session_meta in {}", path.display()))?;
    let title = title_override.map(str::to_string).or_else(|| {
        state
            .first_user_text
            .and_then(|text| derive_title_from_text(&text))
    });
    let id = meta.id;
    Ok(ParsedSession {
        id: id.clone(), title, created_at: meta.timestamp,
        updated_at: state.updated_at.unwrap_or(meta.timestamp),
        messages: state.messages.finish(path), usage: state.usage,
        directory: (!meta.cwd.is_empty()).then(|| {
            let raw = PathBuf::from(&meta.cwd);
            match raw.canonicalize() {
                Ok(canon) => canon,
                Err(_) => {
                    tracing::warn!(target: "codetether_agent::session::codex_import::parse", path = %meta.cwd, "Codex import: cwd does not resolve, storing as-is");
                    raw
                }
            }
        }),
        model: state.model,
    })
}
