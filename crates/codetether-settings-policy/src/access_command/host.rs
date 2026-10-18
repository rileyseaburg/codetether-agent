//! Native session, approval and presentation operations.
use codetether_config::Config;
/// Frontend bindings used by access-mode commands.
/// # Implementors
/// Frontends retain the verifier preference store, active session and approval gate.
pub trait Host {
    /// Try the higher-priority verifier-model command.
    fn verifier_command(&mut self, prompt: &str) -> bool;
    /// Apply loaded config to the current session.
    fn apply_session_config(&mut self, config: &Config);
    /// Release paused requests through the existing approval gate.
    /// # Errors
    /// Returns the gate's existing release error text.
    fn release_all(&mut self) -> Result<usize, String>;
    /// Append the result explanation to chat.
    fn push_system(&mut self, text: String);
    /// Replace frontend status text.
    fn set_status(&mut self, status: String);
    /// Scroll chat to the command result.
    fn scroll_to_bottom(&mut self);
    /// Clear submitted input.
    fn clear_input(&mut self);
}
