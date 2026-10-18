//! Decode and validate the first nonempty archive record.
use anyhow::{Context, Result};
use codetether_codex_records::{CodexSessionMetaPayload, normalize_line};
use std::path::Path;
pub(crate) fn read_first(line: &str, path: &Path) -> Result<Option<CodexSessionMetaPayload>> {
    match normalize_line(line, None)
        .with_context(|| format!("Failed to parse {}", path.display()))?
    {
        Some(record) if record.kind == "session_meta" => {
            let payload: CodexSessionMetaPayload = serde_json::from_value(record.payload)
                .with_context(|| format!("Invalid session_meta in {}", path.display()))?;
            Ok(Some(payload))
        }
        _ => Ok(None),
    }
}
