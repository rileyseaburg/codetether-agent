//! Builds the Responses `reasoning` object.
//!
//! Codex sends both `effort` and `summary`. The `summary` field is the reason its
//! streams stay warm: without it a high-effort turn emits no SSE bytes at all
//! until the final answer begins.
//!
//! Measured against the live ChatGPT Codex backend (`gpt-5.6-luna`, effort=high):
//!
//! | request | first visible token | total |
//! |---|---|---|
//! | no `summary`  | 47.75s | 57.66s |
//! | `summary: auto` | 3.94s | 105.64s |
//!
//! Nearly 48 seconds of dead air is long enough to look like a hung or dead
//! stream to keepalive/idle supervision, so requesting reasoning summaries is a
//! correctness concern and not only a UX improvement.
//!
//! Upstream reference: `codex-rs/core/src/client.rs` (`build_reasoning`).

use serde_json::{Value, json};

/// Reasoning summary mode requested for Codex Responses turns.
///
/// `auto` lets the backend choose summary granularity, which is what the Codex
/// CLI requests by default.
const SUMMARY_AUTO: &str = "auto";

/// Builds the `reasoning` request object for a resolved effort level.
///
/// # Arguments
///
/// * `effort` - Wire value for `reasoning.effort` (for example `"high"`).
///
/// # Returns
///
/// A JSON object carrying `effort` and `summary`.
pub(super) fn reasoning_object(effort: &str) -> Value {
    json!({ "effort": effort, "summary": SUMMARY_AUTO })
}
