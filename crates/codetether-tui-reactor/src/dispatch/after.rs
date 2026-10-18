//! Work drained after a nonterminal event.
use std::future::Future;
/// Post-event application work.
/// # Implementors
/// Native adapters drain updates, queued prompts and bus inbox tasks.
pub trait After {
    /// Drain background updates before measuring visible changes.
    fn drain(&mut self) -> impl Future<Output = ()>;
    /// Submit queued prompts when possible.
    fn queued(&mut self) -> impl Future<Output = ()>;
    /// Trigger the next pending bus inbox task.
    fn inbox(&mut self) -> impl Future<Output = ()>;
}
