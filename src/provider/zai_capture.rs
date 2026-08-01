//! Opt-in capture of raw Z.AI SSE response bodies.
//!
//! The `stream ended without producing any content` failure is intermittent and
//! has resisted reproduction from short CLI conversations: it appears in long
//! TUI sessions, after tool calls, sometimes on a one-word continuation such as
//! "ok". Without the bytes we can only guess whether the provider sent nothing,
//! sent an error frame we discard, or sent deltas we fail to parse.
//!
//! Enabled by setting `CODETETHER_ZAI_CAPTURE_DIR`. Disabled by default: bodies
//! contain full prompts and must never be written implicitly.
//!
//! Mirrors `provider::gemini_web::capture`, which made the Gemini frame-slot
//! corruption diagnosable.

#[path = "zai_capture_fault.rs"]
pub mod fault;

use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Environment variable naming the capture directory.
pub const CAPTURE_DIR_ENV: &str = "CODETETHER_ZAI_CAPTURE_DIR";

/// Appends one raw SSE fragment to this process's capture file.
///
/// Returns the fragment unchanged so callers can capture inline without
/// restructuring the stream. Capture is best-effort and never fails a request.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::zai::capture::record;
///
/// // Disabled without the env var: the input is passed straight through.
/// assert_eq!(record("data: {}\n"), "data: {}\n");
/// ```
pub fn record(fragment: &str) -> &str {
    if let Ok(dir) = std::env::var(CAPTURE_DIR_ENV) {
        append(&dir, fragment);
    }
    fragment
}

/// Appends `fragment` to a per-process capture file inside `dir`.
pub(crate) fn append(dir: &str, fragment: &str) {
    let dir = PathBuf::from(dir.trim());
    if dir.as_os_str().is_empty() || std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let path = dir.join(format!("zai_{}_{stamp}.sse", std::process::id()));
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = file.write_all(fragment.as_bytes());
    }
}

#[cfg(test)]
#[path = "zai_capture_tests.rs"]
mod tests;
