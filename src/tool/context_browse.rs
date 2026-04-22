//! `context_browse`: expose the session transcript as real turn files.
//!
//! ## Meta-Harness filesystem-as-history (Phase B step 21)
//!
//! Lee et al. (arXiv:2603.28052) demonstrate that agents given raw
//! *execution traces* in a filesystem — retrievable via the grep /
//! cat tools they already have — outperform agents given lossy
//! summaries by **+15 accuracy points** on text classification (their
//! Table 3). The ablation is blunt: scores-only 34.6 median, scores +
//! summary 34.9 (essentially unchanged), full traces 50.0. Summaries
//! "may even hurt by compressing away diagnostically useful details".
//!
//! This tool gives the agent the same primitive over *its own past
//! turns*. Every entry in [`Session::messages`] is materialized as a file
//! under the workspace data dir:
//!
//! ```text
//! .codetether-agent/history/<session-id>/turn-NNNN-<role>.md
//! ```
//!
//! The agent's existing Shell / Read / Grep tools can browse those files
//! directly after this tool materializes them; this tool is the *list +
//! locate* layer on top of that directory.
//!
//! ## What it does
//!
//! * `list_turns` (default): returns the canonical paths for every
//!   turn in the current session's history, one per line.
//! * `show_turn`: returns the text body of a single turn by index.
//!
//! For a minimal first cut we surface the transcript from memory via
//! [`Session::load`] — the disk format already matches what the agent
//! would want to grep over. A future commit wires the same namespace
//! to the MinIO-backed pointer resolver for long-horizon archives.
//!
//! ## Invariants
//!
//! * Read-only. This tool never mutates history.
//! * Paths are stable: given an immutable `session.messages`, a path
//!   refers to the same turn across runs.
//!
//! ## Examples
//!
//! ```rust
//! use std::path::Path;
//!
//! use codetether_agent::tool::context_browse::{ContextBrowseAction, format_turn_path, parse_browse_action};
//! use serde_json::json;
//!
//! assert_eq!(
//!     format_turn_path(Path::new("/tmp/history/abc-123"), 7, "user"),
//!     Path::new("/tmp/history/abc-123/turn-0007-user.md"),
//! );
//!
//! let action = parse_browse_action(&json!({})).unwrap();
//! assert!(matches!(action, ContextBrowseAction::ListTurns));
//!
//! let action = parse_browse_action(&json!({"action": "show_turn", "turn": 3})).unwrap();
//! assert!(matches!(action, ContextBrowseAction::ShowTurn { turn: 3 }));
//! ```

use super::{Tool, ToolResult};
use crate::session::Fault;
use crate::session::Session;
use crate::session::history_files::{materialize_session_history, render_turn};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

/// Parsed form of the JSON `args` payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextBrowseAction {
    /// Default action — enumerate every turn as a virtual path.
    ListTurns,
    /// Return the text body of the turn at `turn`.
    ShowTurn { turn: usize },
}

/// Parse a tool-args [`Value`] into a typed [`ContextBrowseAction`].
///
/// Factored out so the parsing logic is unit-testable without routing
/// through the full [`Tool`] trait machinery.
pub fn parse_browse_action(args: &Value) -> Result<ContextBrowseAction, String> {
    let action = args.get("action").and_then(Value::as_str).unwrap_or("list");
    match action {
        "list" | "list_turns" => Ok(ContextBrowseAction::ListTurns),
        "show" | "show_turn" => {
            let turn = args
                .get("turn")
                .and_then(Value::as_u64)
                .ok_or_else(|| "`turn` (integer) is required for show_turn".to_string())?
                as usize;
            Ok(ContextBrowseAction::ShowTurn { turn })
        }
        other => Err(format!("unknown action: {other}")),
    }
}

/// Produce the canonical materialized filesystem path for a turn.
pub fn format_turn_path(session_dir: &Path, turn: usize, role: &str) -> PathBuf {
    crate::session::history_files::format_turn_path(session_dir, turn, role)
}

/// Render a listing of materialized paths, one per line.
fn render_listing(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Meta-Harness filesystem-as-history tool.
pub struct ContextBrowseTool;

#[async_trait]
impl Tool for ContextBrowseTool {
    fn id(&self) -> &str {
        "context_browse"
    }

    fn name(&self) -> &str {
        "ContextBrowse"
    }

    fn description(&self) -> &str {
        "BROWSE YOUR OWN HISTORY AS A FILESYSTEM (Meta-Harness, \
         arXiv:2603.28052). Materializes real files under \
         `.codetether-agent/history/<session-id>/turn-NNNN-<role>.md` \
         — one per turn in the canonical transcript — and returns the \
         body of any specific turn on request. Use this when the active \
         context doesn't have what you need but you suspect it was said earlier. \
         Distinct from `session_recall` (RLM-summarised archive) and \
         `memory` (curated notes). Actions: `list` (default) or \
         `show_turn` with an integer `turn`."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "list_turns", "show", "show_turn"],
                    "description": "Operation: 'list' enumerates turn paths, \
                                    'show_turn' returns one turn body."
                },
                "turn": {
                    "type": "integer",
                    "description": "Zero-based turn index, required with \
                                    action='show_turn'.",
                    "minimum": 0
                }
            },
            "required": []
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let action = match parse_browse_action(&args) {
            Ok(a) => a,
            Err(e) => return Ok(ToolResult::error(e)),
        };
        let session = match latest_session_for_cwd().await {
            Ok(Some(s)) => s,
            Ok(None) => {
                return Ok(fault_result(
                    Fault::NoMatch,
                    "No session found for the current workspace.",
                ));
            }
            Err(e) => {
                return Ok(fault_result(
                    Fault::BackendError {
                        reason: e.to_string(),
                    },
                    format!("failed to load session: {e}"),
                ));
            }
        };
        let paths = match materialize_session_history(&session).await {
            Ok(paths) => paths,
            Err(err) => {
                return Ok(fault_result(
                    Fault::BackendError {
                        reason: err.to_string(),
                    },
                    format!("failed to materialize history files: {err}"),
                ));
            }
        };
        let messages = session.history();
        match action {
            ContextBrowseAction::ListTurns => Ok(ToolResult::success(render_listing(&paths))
                .with_metadata("session_id", json!(session.id))
                .truncate_to(super::tool_output_budget())),
            ContextBrowseAction::ShowTurn { turn } => match messages.get(turn) {
                Some(msg) => Ok(ToolResult::success(render_turn(msg))
                    .with_metadata(
                        "path",
                        json!(
                            paths
                                .get(turn)
                                .map(|path| path.display().to_string())
                                .unwrap_or_else(String::new)
                        ),
                    )
                    .truncate_to(super::tool_output_budget())),
                None => Ok(ToolResult::error(format!(
                    "turn {turn} out of range (have {} entries)",
                    messages.len()
                ))),
            },
        }
    }
}

fn fault_result(fault: Fault, output: impl Into<String>) -> ToolResult {
    let code = fault.code();
    let detail = fault.to_string();
    ToolResult::error(output)
        .with_metadata("fault_code", json!(code))
        .with_metadata("fault_detail", json!(detail))
}

/// Resolve the session this tool should browse.
///
/// For Phase B v1 we browse the most recent session rooted at the
/// current working directory — the same one `session_recall` uses.
/// The agent-owning session is not yet threaded through the Tool
/// trait; a future commit will switch to the in-memory live session
/// once that signature lands.
///
/// Distinguishes "no sessions exist yet for this workspace" (returns
/// `Ok(None)`) from real I/O or parse errors (returns `Err(...)`) so
/// the caller can surface a `ToolResult::error` rather than silently
/// masking a broken session store.
async fn latest_session_for_cwd() -> Result<Option<Session>> {
    let cwd = std::env::current_dir().ok();
    let workspace = cwd.as_deref();
    match Session::last_for_directory(workspace).await {
        Ok(s) => Ok(Some(s)),
        Err(err) => {
            let msg = err.to_string().to_lowercase();
            // `Session::last_for_directory` returns an error when no sessions
            // exist for the workspace. Treat those as `Ok(None)` so the tool
            // can report "no session found"; bubble everything else up.
            if msg.contains("no session")
                || msg.contains("not found")
                || msg.contains("no such file")
            {
                tracing::debug!(%err, "context_browse: no session for workspace");
                Ok(None)
            } else {
                tracing::warn!(%err, "context_browse: failed to load latest session");
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn parse_browse_action_defaults_to_list() {
        let action = parse_browse_action(&json!({})).unwrap();
        assert!(matches!(action, ContextBrowseAction::ListTurns));
    }

    #[test]
    fn parse_browse_action_requires_turn_for_show() {
        let err = parse_browse_action(&json!({"action": "show_turn"})).unwrap_err();
        assert!(err.contains("turn"));
    }

    #[test]
    fn parse_browse_action_rejects_unknown() {
        let err = parse_browse_action(&json!({"action": "truncate"})).unwrap_err();
        assert!(err.contains("unknown"));
    }

    #[test]
    fn format_turn_path_uses_real_filesystem_paths() {
        let path0 = format_turn_path(Path::new("/tmp/history/sid"), 0, "user");
        let path = format_turn_path(Path::new("/tmp/history/sid"), 7, "assistant");
        assert_eq!(path0, PathBuf::from("/tmp/history/sid/turn-0000-user.md"));
        assert_eq!(
            path,
            PathBuf::from("/tmp/history/sid/turn-0007-assistant.md")
        );
    }

    #[test]
    fn render_listing_emits_one_path_per_turn() {
        let listing = render_listing(&[
            PathBuf::from("/tmp/history/sid/turn-0000-user.md"),
            PathBuf::from("/tmp/history/sid/turn-0001-assistant.md"),
        ]);
        assert_eq!(listing.lines().count(), 2);
        assert!(listing.contains("/tmp/history/sid/turn-0000-user.md"));
        assert!(listing.contains("/tmp/history/sid/turn-0001-assistant.md"));
    }
}
