//! Major terminal application views.
use serde::{Deserialize, Serialize};

/// Top-level screens available in the terminal UI.
///
/// `ViewMode` records which major panel is currently active. It is serialized
/// with session or UI state so the interface can preserve and restore the
/// selected screen when appropriate.
///
/// Variants correspond to user-facing views such as chat, saved sessions,
/// swarm execution, Ralph automation, bus logs, model selection, settings,
/// language tooling, latency metrics, protocol inspection, file picking,
/// audit logs, and git status.
///
/// # Examples
///
/// ```
/// use codetether_tui_models::ViewMode;
///
/// let mode = ViewMode::Chat;
/// assert_eq!(mode, ViewMode::Chat);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewMode {
    /// Main conversation view for prompts, assistant responses, and tool output.
    Chat,
    /// Saved-session browser and session switching view.
    Sessions,
    /// Parallel swarm task execution and result view.
    Swarm,
    /// Ralph PRD-driven autonomous development view.
    Ralph,
    /// Agent bus event and collaboration log view.
    Bus,
    /// Parent/child sub-agent deployment and reporting dashboard.
    Subagents,
    /// Model picker and provider model selection view.
    Model,
    /// User-configurable TUI and runtime settings view.
    Settings,
    /// Language Server Protocol diagnostics and symbol tooling view.
    Lsp,
    /// Recursive Language Model processing view.
    Rlm,
    /// Request, tool, and streaming latency metrics view.
    Latency,
    /// Transport-health view: TCP_INFO probe (RTT, cwnd, retransmits).
    Transport,
    /// Protocol-level message inspection view.
    Protocol,
    /// Workspace file picker view.
    FilePicker,
    /// Inspector view for detailed application or message state.
    Inspector,
    /// Audit trail view for security and operational events.
    Audit,
    /// Git status and repository change view.
    Git,
    /// Audit-loop view: implement → audit → retry cycle visualization.
    AuditLoop,
    /// In-TUI text editor view for editing workspace files.
    Editor,
}
