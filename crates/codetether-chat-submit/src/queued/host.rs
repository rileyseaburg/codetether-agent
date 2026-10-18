use std::future::Future;
/// Goal input delivered before queued chat submission.
/// # Implementors
/// Native goal-command and answer handlers retain their current side effects.
pub trait Commands {
    /// Drain queued explicit goal-answer commands.
    fn commands(&mut self) -> impl Future<Output = ()>;
    /// Deliver answers waiting for an available session.
    fn answers(&mut self) -> impl Future<Output = ()>;
}
/// One-shot prompt queue and execution gates.
/// # Implementors
/// Native adapters retain queue storage and durable goal lookups.
pub trait Queue {
    /// Whether a request is still being processed.
    fn processing(&self) -> bool;
    /// Whether a one-shot prompt is waiting.
    fn has_pending(&self) -> bool;
    /// Whether answer review holds the current goal.
    fn held(&self) -> bool;
    /// Remove and return the queued prompt.
    fn take(&mut self) -> Option<String>;
}
/// Mutable input fields populated before ordinary Enter dispatch.
pub struct State<'a> {
    /// Current editable text.
    pub input: &'a mut String,
    /// Cursor position in characters.
    pub cursor: &'a mut usize,
    /// Visible status message.
    pub status: &'a mut String,
}
/// Native input and normal Enter dispatch.
/// # Implementors
/// UI adapters retain the existing submission entrypoint.
pub trait Input {
    /// Borrow input fields to populate from the queued prompt.
    fn input(&mut self) -> State<'_>;
    /// Dispatch the populated input through the normal Enter handler.
    fn enter(&mut self) -> impl Future<Output = ()>;
}
