//! Stream archive lines and dispatch decoded records in source order.
use crate::{parsed::ParsedSession, state::State};
use anyhow::{Context, Result};
use codetether_codex_records::normalize_line;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
/// Parse an archive into a bounded provider transcript and session metadata.
/// # Arguments
/// * `path` — Source JSONL archive.
/// * `title_override` — Indexed title preferred over the first user message.
/// # Returns
/// Parsed metadata, latest token usage and at most 4,000 recent messages.
/// # Errors
/// Propagates file reads, invalid JSON/records and missing session metadata.
/// # Panics
/// Legacy title truncation panics if byte 57 splits a long UTF-8 title.
/// # Examples
/// ```no_run
/// let parsed = codetether_codex_transcript::parse_codex_session_from_path(
///     std::path::Path::new("rollout.jsonl"), None)?;
/// assert!(parsed.messages.len() <= 4000);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn parse_codex_session_from_path(
    path: &Path,
    title_override: Option<&str>,
) -> Result<ParsedSession> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open Codex session {}", path.display()))?;
    let mut state = State::default();
    for line in BufReader::new(file).lines() {
        let line = line.with_context(|| format!("Failed to read {}", path.display()))?;
        if line.trim().is_empty() {
            continue;
        }
        let session_ts = state.session_meta.as_ref().map(|m| m.timestamp);
        let record = match normalize_line(&line, session_ts)
            .with_context(|| format!("Failed to parse {}", path.display()))?
        {
            Some(r) => r,
            None => continue,
        };
        state.updated_at = Some(
            state
                .updated_at
                .map_or(record.timestamp, |current| current.max(record.timestamp)),
        );
        crate::consume::consume(&mut state, record)?;
    }
    crate::finish::finish(state, path, title_override)
}
