/// Native sources and handlers for background updates.
///
/// # Implementors
/// The TUI adapter owns borrowed state and receivers for an event-loop iteration.
pub trait Host {
    /// Completion notice produced by the native session runtime.
    type Notice;
    /// Drain model catalog refreshes.
    fn models(&mut self);
    /// Ingest pending bus messages.
    fn bus(&mut self);
    /// Queue worker tasks received from the bridge.
    fn queue_workers(&mut self);
    /// Drain a bounded batch of session events.
    fn events(&mut self) -> impl Future<Output = ()>;
    /// Take a ready completion notice without waiting.
    fn next_notice(&mut self) -> Option<Self::Notice>;
    /// Drain session events that must precede a completion notice.
    fn before_notice(&mut self) -> impl Future<Output = ()>;
    /// Apply one completion notice to native state.
    fn notice(&mut self, notice: Self::Notice) -> impl Future<Output = ()>;
}
