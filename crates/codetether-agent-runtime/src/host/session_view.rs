//! Read-only session state and prior-context policy.
use codetether_agent_core::ToolUse;
use codetether_provider_core::{Message, Usage};
use codetether_tool_core::ToolResult;
use std::path::Path;
/// Borrow session state without copying its authoritative metadata.
/// # Implementors
/// Native sessions retain their current history and access-policy behavior.
pub trait SessionView: Send + Sync + 'static {
    /// Native session used by all completion and replay services.
    type Session: Send + Sync;
    /// Borrow the configured directory.
    fn directory(session: &Self::Session) -> Option<&Path>;
    /// Borrow message history.
    fn messages(session: &Self::Session) -> &[Message];
    /// Borrow recorded tool usage.
    fn tool_uses(session: &Self::Session) -> &[ToolUse];
    /// Borrow token accounting.
    fn usage(session: &Self::Session) -> &Usage;
    /// Check whether a prior-context tool may be advertised.
    fn available(session: &Self::Session, name: &str) -> bool;
    /// Return a prior-context execution denial when applicable.
    fn prior_blocked(session: &Self::Session, name: &str, args: &str) -> Option<ToolResult>;
}
