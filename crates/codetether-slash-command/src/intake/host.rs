use std::future::Future;
/// Native command handlers and session availability.
/// # Implementors
/// UI adapters retain the mutable session borrow and all actual command effects.
pub trait Host {
    /// Run shell-prefix input; true consumes it.
    fn shell(&mut self, prompt: &str) -> impl Future<Output = bool>;
    /// Run approval commands; true consumes the input.
    fn approval(&mut self, prompt: &str) -> bool;
    /// Run mux commands; true consumes the input.
    fn mux(&mut self, prompt: &str) -> impl Future<Output = bool>;
    /// Borrow and dispatch a live session; false means none is available.
    fn session(&mut self, prompt: &str) -> impl Future<Output = bool>;
    /// Run UI-only commands while the session is unavailable.
    fn session_free(&mut self, prompt: &str) -> bool;
    /// Clear input after a handled session-free command.
    fn clear_input(&mut self);
    /// Show status for an unavailable session command.
    fn status(&mut self, message: String);
}
