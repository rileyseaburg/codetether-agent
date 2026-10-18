use std::future::Future;
/// Prompt capture and command/agent routing after interception.
/// # Implementors
/// Native handlers retain history, mention policy and command execution.
pub trait Routing {
    /// Capture raw prompt text and record native input history.
    fn capture(&mut self) -> String;
    /// Route to a managed agent; true consumes the input.
    fn managed(&mut self, prompt: &str) -> impl Future<Output = bool>;
    /// Resolve mention routing for the current selected agent.
    fn mentions(&self, prompt: &str) -> String;
    /// Process continuation commands; true consumes the input.
    fn continuation(&mut self, prompt: &str) -> impl Future<Output = bool>;
    /// Process slash commands; true consumes the input.
    fn slash(&mut self, prompt: &str) -> impl Future<Output = bool>;
}
