use anyhow::{Context, Result};
use codetether_codex_records::CodexSessionMetaPayload;
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
/// Read canonical session metadata with legacy directory recovery.
/// # Arguments
/// * `path` — Archive JSONL file to inspect.
/// # Returns
/// Decoded metadata, or None for an unrecognized first record.
/// # Errors
/// Propagates file reads and first-record decoding errors.
/// # Examples
/// ```no_run
/// let result = codetether_codex_discovery::read_session_meta(std::path::Path::new("rollout.jsonl"));
/// assert!(result.is_ok());
/// ```
pub fn read_session_meta(path: &Path) -> Result<Option<CodexSessionMetaPayload>> {
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
            match super::meta_first::read_first(&line, path)? {
                Some(payload) => {
                    if !payload.cwd.is_empty() {
                        return Ok(Some(payload));
                    }
                    meta = Some(payload);
                    continue;
                }
                None => return Ok(None),
            }
        }

        // Legacy meta: scan subsequent lines for <environment_context> cwd.
        if meta.is_some() {
            if let Some(cwd) = super::meta_recover::recover(&line) {
                let mut payload = meta.take().expect("meta is Some");
                payload.cwd = cwd;
                return Ok(Some(payload));
            }
        }
    }

    // End of file: return legacy meta with whatever cwd we recovered (possibly empty).
    Ok(meta)
}
