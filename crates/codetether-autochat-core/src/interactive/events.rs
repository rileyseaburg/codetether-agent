//! Autochat UI event types for the TUI event loop.

/// Events emitted by the relay worker, parameterized by session-event payload.
///
/// # Examples
/// ```
/// use codetether_autochat_core::interactive::events::AutochatUiEvent;
/// let event = AutochatUiEvent::<()>::Progress("thinking".into());
/// match event {
///     AutochatUiEvent::Progress(text) => assert_eq!(text, "thinking"),
///     _ => unreachable!(),
/// }
/// ```
#[derive(Debug)]
pub enum AutochatUiEvent<E> {
    /// Incremental progress update (shown in status bar).
    Progress(String),
    /// System-level message to display in chat.
    SystemMessage(String),
    /// An agent in the relay produced a session event.
    AgentEvent { agent_name: String, event: Box<E> },
    /// Relay completed with a summary and optional OKR linkage.
    Completed {
        summary: String,
        okr_id: Option<String>,
        okr_run_id: Option<String>,
        relay_id: Option<String>,
    },
}
