use crate::{Call, LoopState};
/// Native access to the existing pre-execution guards and per-turn state.
/// # Implementors
/// The prompt runner delegates blocked requests to its established guard path.
pub trait Policy {
    /// Return a rejection explanation before executing a call.
    fn blocked(&mut self, call: &Call) -> Option<String>;
    /// Access the current turn's decline guard and counters.
    fn progress(&mut self) -> &mut LoopState;
    /// Apply the empty-code-search recovery guard after publication.
    fn codesearch_guard(&mut self, step: usize) -> bool;
}
