//! Opt-in capture of raw Gemini Web response bodies.
//!
//! The Gemini Web endpoint is undocumented and its frame layout has drifted:
//! `response_text::event_text` reads a single slot (`[4][0][1][0]`), and when
//! that slot changes meaning the provider silently surfaces the wrong text —
//! observed as leaked model reasoning ("Let's respond warmly...") and as
//! Gemini's own "Sorry, something went wrong." appearing as the assistant reply.
//!
//! Diagnosing that requires the real bytes, so this writes them to a file when
//! `CODETETHER_GEMINI_WEB_CAPTURE_DIR` is set. It is disabled by default: bodies
//! contain full prompts and must never be written implicitly.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Environment variable naming the capture directory.
pub(super) const CAPTURE_DIR_ENV: &str = "CODETETHER_GEMINI_WEB_CAPTURE_DIR";

/// Writes `body` to the capture directory when capture is enabled.
///
/// Returns the written path, or `None` when capture is disabled or the write
/// fails. Capture is best-effort diagnostics and never fails a request.
pub(super) fn record(model: &str, body: &str) -> Option<PathBuf> {
    let dir = std::env::var(CAPTURE_DIR_ENV).ok()?;
    let dir = PathBuf::from(dir.trim());
    if dir.as_os_str().is_empty() {
        return None;
    }
    std::fs::create_dir_all(&dir).ok()?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default();
    let safe_model: String = model
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    let path = dir.join(format!("gemini_{safe_model}_{stamp}.txt"));
    std::fs::write(&path, body).ok()?;
    tracing::debug!(path = %path.display(), "Captured Gemini Web response body");
    Some(path)
}
