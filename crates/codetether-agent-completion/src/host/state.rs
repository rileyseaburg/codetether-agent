//! Native agent configuration, session accounting and prompt composition.
use crate::*;
/// Lend existing fields and preserve native history mutation methods.
/// # Implementors
/// Native agents already implement the replay host used by this interface.
pub trait State: codetether_agent_replay::Host {
    /// Borrow configured model settings and step limit.
    fn info(&self) -> &AgentInfo;
    /// Borrow the configured provider.
    fn provider(&self) -> &Arc<dyn Provider>;
    /// Read all advertised tool definitions in registry order.
    fn definitions(&self) -> Vec<ToolDefinition>;
    /// Apply the native session's prior-context tool availability policy.
    fn tool_available(session: &Self::Session, name: &str) -> bool;
    /// Compose the persona with native goal and peer instructions.
    fn system_prompt(&self, session: &Self::Session) -> String;
    /// Append the human message with the existing session semantics.
    fn add_human_message(session: &mut Self::Session, message: Message);
    /// Borrow the recorded tool usage in its existing order.
    fn tool_uses(session: &Self::Session) -> &[ToolUse];
    /// Borrow cumulative provider usage.
    fn usage(session: &Self::Session) -> &Usage;
}
