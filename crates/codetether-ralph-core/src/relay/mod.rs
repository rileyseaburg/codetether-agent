//! Relay turn ordering and convergence independent of live sessions.
//!
//! [RelayProgress] tracks handoff text and termination while [sender] selects
//! the prior participant. The host executes agents and records the turn count.
mod progress;
mod sender;
pub use sender::sender;
/// Mutable relay output and convergence evidence.
///
/// # Examples
/// ```
/// let progress = codetether_ralph_core::relay::RelayProgress {
///     baton: "task".into(), previous_normalized: None, convergence_hits: 0, turns: 0,
/// };
/// assert_eq!(progress.turns, 0);
/// ```

pub struct RelayProgress {
    pub baton: String,
    pub previous_normalized: Option<String>,
    pub convergence_hits: usize,
    pub turns: usize,
}
