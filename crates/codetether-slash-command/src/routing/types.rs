//! Recognized command groups in interactive dispatch order.
/// Commands handled before conditional relay and session dispatch.
/// # Examples
/// ```
/// use codetether_slash_command::routing::{leading, LeadingCommand};
/// assert!(matches!(leading("/image pic.png"), Some((LeadingCommand::Image, "pic.png"))));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeadingCommand {
    /// Prepare a goal for approval.
    Go,
    /// Attach an image.
    Image,
    /// Attach a file or open the picker.
    File,
    /// Open the embedded editor.
    Edit,
    /// Set automatic edit application.
    AutoApply,
    /// Set network access.
    Network,
    /// Set command autocomplete.
    Autocomplete,
    /// Ask an ephemeral side question.
    Ask,
    /// Display context status.
    Context,
    /// Manage MCP connections.
    Mcp,
}
/// Session-related command dispatch group.
/// # Examples
/// ```
/// use codetether_slash_command::routing::{session, SessionCommand};
/// assert!(matches!(session("/undo 2"), Some((SessionCommand::Undo, "2"))));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionCommand {
    /// Scan or execute forage work.
    Forage,
    /// Manage the session goal.
    Goal,
    /// Manage A2A peers.
    A2a,
    /// Undo user turns.
    Undo,
    /// Detach the active task.
    Detach,
    /// Fork the current conversation.
    Fork,
}
