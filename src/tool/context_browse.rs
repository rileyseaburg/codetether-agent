//! `context_browse`: expose the session transcript as virtual files.
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
//! turns*. Every entry in [`Session::messages`] is exposed as a path
//! of the form:
//!
//! ```text
//! session://<session-id>/turn-NNNN-<role>.md
//! ```
//!
//! The agent's existing Shell / Read / Grep tools can already browse
//! these paths once Phase A's history-sink backing store is populated;
//! this tool is the *list + locate* layer on top of it.
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
//! use codetether_agent::tool::context_browse::{
//!     ContextBrowseAction, format_turn_path, parse_browse_action,
//! };
//! use serde_json::json;
//!
//! assert_eq!(
//!     format_turn_path("abc-123", 7, "user"),
//!     "session://abc-123/turn-0007-user.md",
//! );
//!
//! let action = parse_browse_action(&json!({})).unwrap();
//! assert!(matches!(action, ContextBrowseAction::ListTurns));
//!
//! let action = parse_browse_action(&json!({"action": "show_turn", "turn": 3})).unwrap();
//! assert!(matches!(action, ContextBrowseAction::ShowTurn { turn: 3 }));
//! ```

use super::{Tool, ToolResult};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

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

/// Produce the canonical virtual path for a turn.
pub fn format_turn_path(session_id: &str, turn: usize, role: &str) -> String {
    format!("session://{session_id}/turn-{turn:04}-{role}.md")
}

/// Lower-case role label used in the path.
fn role_label(role: &Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

/// Render a message body as the text the agent would see if it opened
/// the virtual file. Text and tool-result parts are concatenated in
/// order; non-textual parts are replaced by a short placeholder so the
/// path stays readable.
fn render_turn(msg: &Message) -> String {
    let mut buf = String::new();
    for part in &msg.content {
        if !buf.is_empty() {
            buf.push_str("\n\n");
        }
        match part {
            ContentPart::Text { text } => buf.push_str(text),
            ContentPart::ToolResult { tool_call_id, content } => {
                buf.push_str(&format!("[tool_result tool_call_id={tool_call_id}]\n"));
                buf.push_str(content);
            }
            ContentPart::ToolCall { name, arguments, .. } => {
                buf.push_str(&format!("[tool_call {name}]\n{arguments}"));
            }
            ContentPart::Image { url, .. } => {
                buf.push_str(&format!("[image {url}]"));
            }
            ContentPart::File { path, .. } => {
                buf.push_str(&format!("[file {path}]"));
            }
            ContentPart::Thinking { text } => {
                buf.push_str(&format!("[thinking]\n{text}"));
            }
        }
    }
    buf
}

/// Render a listing of virtual paths, one per line.
fn render_listing(session_id: &str, messages: &[Message]) -> String {
    let mut out = String::new();
    for (idx, msg) in messages.iter().enumerate() {
        out.push_str(&format_turn_path(session_id, idx, role_label(&msg.role)));
        out.push('\n');
    }
    out
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
         arXiv:2603.28052). Lists virtual paths like \
         `session://<id>/turn-NNNN-<role>.md` — one per turn in the \
         canonical transcript — and returns the body of any specific \
         turn on request. Use this when the active context doesn't \
         have what you need but you suspect it was said earlier. \
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
                return Ok(ToolResult::error(
                    "No session found for the current workspace.",
                ));
            }
            Err(e) => return Ok(ToolResult::error(format!("failed to load session: {e}"))),
        };
        let messages = session.history();
        match action {
            ContextBrowseAction::ListTurns => Ok(ToolResult::success(render_listing(
                &session.id,
                messages,
            ))
            .truncate_to(super::tool_output_budget())),
            ContextBrowseAction::ShowTurn { turn } => match messages.get(turn) {
                Some(msg) => Ok(ToolResult::success(render_turn(msg))
                    .truncate_to(super::tool_output_budget())),
                None => Ok(ToolResult::error(format!(
                    "turn {turn} out of range (have {} entries)",
                    messages.len()
                ))),
            },
        }
    }
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
    fn format_turn_path_pads_index_to_four_digits() {
        assert_eq!(
            format_turn_path("abc-123", 0, "user"),
            "session://abc-123/turn-0000-user.md"
        );
        assert_eq!(
            format_turn_path("abc-123", 9999, "assistant"),
            "session://abc-123/turn-9999-assistant.md"
        );
    }

    fn text(role: Role, s: &str) -> Message {
        Message {
            role,
            content: vec![ContentPart::Text {
                text: s.to_string(),
            }],
        }
    }

    #[test]
    fn render_listing_emits_one_path_per_turn() {
        let msgs = vec![
            text(Role::User, "hi"),
            text(Role::Assistant, "hello"),
            text(Role::User, "more"),
        ];
        let listing = render_listing("sid", &msgs);
        assert_eq!(listing.lines().count(), 3);
        assert!(listing.contains("session://sid/turn-0000-user.md"));
        assert!(listing.contains("session://sid/turn-0001-assistant.md"));
        assert!(listing.contains("session://sid/turn-0002-user.md"));
    }

    #[test]
    fn render_turn_preserves_text_body() {
        let body = render_turn(&text(Role::User, "quick brown fox"));
        assert_eq!(body, "quick brown fox");
    }
}
