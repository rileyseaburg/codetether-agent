//! Dispatch classified session commands through frontend-owned actions.
use crate::routing::{SessionCommand, session as classify};
/// Native operations requiring the active session context.
/// # Implementors
/// Frontends retain their session, workspace and application command actions.
#[allow(async_fn_in_trait)]
pub trait Host {
    /// Handle the forage command.
    fn forage(&mut self, rest: &str);
    /// Handle a human goal command.
    async fn goal(&mut self, rest: &str);
    /// Handle live A2A attachment commands.
    fn a2a(&mut self, rest: &str);
    /// Undo requested session turns.
    async fn undo(&mut self, rest: &str);
    /// Detach session work using the existing frontend workflow.
    async fn detach(&mut self, rest: &str);
    /// Fork the active session.
    async fn fork(&mut self, rest: &str);
}
/// Classify and dispatch a session-bound command.
/// # Arguments
/// * `host` — Native actions for the active session.
/// * `normalized` — Normalized command text.
/// # Returns
/// Whether a session command was recognized and dispatched.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_slash_command::pipeline::session::Host) {
/// let _ = codetether_slash_command::pipeline::session::dispatch(host, "/goal status").await;
/// # }
/// ```
pub async fn dispatch(host: &mut impl Host, normalized: &str) -> bool {
    let Some((command, rest)) = classify(normalized) else {
        return false;
    };
    match command {
        SessionCommand::Forage => {
            host.forage(rest);
        }
        SessionCommand::Goal => {
            host.goal(rest).await;
        }
        SessionCommand::A2a => {
            host.a2a(rest);
        }
        SessionCommand::Undo => {
            host.undo(rest).await;
        }
        SessionCommand::Detach => {
            host.detach(rest).await;
        }
        SessionCommand::Fork => {
            host.fork(rest).await;
        }
    }
    true
}
