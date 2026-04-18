use super::legacy::{extract_cwd_from_env_context, normalize_line};
use super::records::CodexSessionMetaPayload;
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Read just enough of a Codex rollout file to produce the canonical
/// [`CodexSessionMetaPayload`].
///
/// Supports both the modern `{timestamp, type: "session_meta", payload}`
/// envelope and the legacy flat first-line meta. For legacy files without a
/// `cwd` field, this function continues scanning until it finds the first
/// `<environment_context>` user message and extracts the `<cwd>` tag from it.
///
/// Returns `Ok(None)` when the file's first record is neither a modern
/// `session_meta` envelope nor a legacy meta (i.e. not a Codex rollout).
pub(crate) fn read_session_meta(path: &Path) -> Result<Option<CodexSessionMetaPayload>> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open Codex session {}", path.display()))?;

    let mut meta: Option<CodexSessionMetaPayload> = None;
    let mut saw_first = false;

    for line in BufReader::new(file).lines() {
        let line = line.with_context(|| format!("Failed to read {}", path.display()))?;
        if line.trim().is_empty() {
            continue;
        }

        if !saw_first {
            saw_first = true;
            match normalize_line(&line, None)
                .with_context(|| format!("Failed to parse {}", path.display()))?
            {
                Some(record) if record.kind == "session_meta" => {
                    let payload: CodexSessionMetaPayload =
                        serde_json::from_value(record.payload).with_context(|| {
                            format!("Invalid session_meta in {}", path.display())
                        })?;
                    // Modern meta already carries cwd — we're done.
                    if !payload.cwd.is_empty() {
                        return Ok(Some(payload));
                    }
                    meta = Some(payload);
                    continue;
                }
                // First line isn't a session_meta: this isn't a Codex rollout.
                _ => return Ok(None),
            }
        }

        // Legacy meta: scan subsequent lines for <environment_context> cwd.
        if meta.is_some() {
            let value: Value = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(_) => continue, // tolerate mid-file parse errors during cwd recovery
            };
            if let Some(cwd) = extract_cwd_from_env_context(&value) {
                let mut payload = meta.take().expect("meta is Some");
                payload.cwd = cwd;
                return Ok(Some(payload));
            }
        }
    }

    // End of file: return legacy meta with whatever cwd we recovered (possibly empty).
    Ok(meta)
}
