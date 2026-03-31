use super::super::{Session, SessionMetadata};
use super::records::{CodexJsonlRecord, CodexSessionMetaPayload, CodexTurnContextPayload};
use super::response::parse_response_item;
use super::title::derive_title_from_text;
use super::usage::parse_event_msg_usage;
use crate::provenance::ExecutionProvenance;
use crate::provider::Usage;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub(crate) fn parse_codex_session_from_path(
    path: &Path,
    title_override: Option<&str>,
) -> Result<Session> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open Codex session {}", path.display()))?;
    let mut session_meta: Option<CodexSessionMetaPayload> = None;
    let mut updated_at: Option<DateTime<Utc>> = None;
    let mut messages = Vec::new();
    let mut usage = Usage::default();
    let mut model = None;
    let mut first_user_text = None;

    for line in BufReader::new(file).lines() {
        let line = line.with_context(|| format!("Failed to read {}", path.display()))?;
        if line.trim().is_empty() {
            continue;
        }
        let record: CodexJsonlRecord = serde_json::from_str(&line)
            .with_context(|| format!("Failed to parse {}", path.display()))?;
        updated_at =
            Some(updated_at.map_or(record.timestamp, |current| current.max(record.timestamp)));
        match record.kind.as_str() {
            "session_meta" => session_meta = Some(serde_json::from_value(record.payload)?),
            "turn_context" => {
                let ctx: CodexTurnContextPayload = serde_json::from_value(record.payload)?;
                if let Some(next_model) = ctx.model.filter(|value| !value.is_empty()) {
                    model = Some(next_model);
                }
            }
            "response_item" => {
                if let Some(message) = parse_response_item(record.payload, &mut first_user_text)? {
                    messages.push(message);
                }
            }
            "event_msg" => {
                if let Some(next_usage) = parse_event_msg_usage(record.payload)? {
                    usage = next_usage;
                }
            }
            _ => {}
        }
    }

    let meta =
        session_meta.with_context(|| format!("Missing session_meta in {}", path.display()))?;
    let title = title_override
        .map(str::to_string)
        .or_else(|| first_user_text.and_then(|text| derive_title_from_text(&text)));
    let id = meta.id;
    Ok(Session {
        id: id.clone(),
        title,
        created_at: meta.timestamp,
        updated_at: updated_at.unwrap_or(meta.timestamp),
        messages,
        tool_uses: Vec::new(),
        usage,
        agent: "build".to_string(),
        metadata: SessionMetadata {
            directory: (!meta.cwd.is_empty()).then(|| PathBuf::from(meta.cwd)),
            model,
            provenance: Some(ExecutionProvenance::for_session(&id, "build")),
            ..Default::default()
        },
        bus: None,
    })
}
