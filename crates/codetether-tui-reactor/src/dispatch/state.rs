//! Visible-state observation for redraw decisions.
/// State observed before waiting and after dispatch.
/// # Implementors
/// Native adapters project visible state and expose the existing redraw flag.
pub trait State {
    /// Comparable visible-state snapshot.
    type Snapshot: PartialEq;
    /// Capture current visible state.
    fn snapshot(&self) -> Self::Snapshot;
    /// Access the existing redraw flag.
    fn needs_redraw(&mut self) -> &mut bool;
}
