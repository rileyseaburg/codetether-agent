use super::records::{CodexJsonlRecord, CodexSessionMetaPayload};
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub(crate) fn read_session_meta(path: &Path) -> Result<Option<CodexSessionMetaPayload>> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open Codex session {}", path.display()))?;

    for line in BufReader::new(file).lines() {
        let line = line.with_context(|| format!("Failed to read {}", path.display()))?;
        if line.trim().is_empty() {
            continue;
        }
        let record: CodexJsonlRecord = serde_json::from_str(&line)
            .with_context(|| format!("Failed to parse {}", path.display()))?;
        if record.kind != "session_meta" {
            return Ok(None);
        }
        return serde_json::from_value(record.payload)
            .with_context(|| format!("Invalid session_meta in {}", path.display()))
            .map(Some);
    }

    Ok(None)
}
