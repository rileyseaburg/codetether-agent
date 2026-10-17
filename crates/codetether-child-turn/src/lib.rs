//! Child-turn finalization, detached collection and first-turn coordination.
//!
//! [finalize] preserves session-save and terminal-publication ordering.
//! [dispatch] keeps an execution guard alive during background collection;
//! [kick_off] starts and settles a newly spawned child's initial turn.
//! Native [FinalizationHost] and [TurnHost] operations retain storage, steering,
//! event projection and mailbox completion behavior.
mod detached;
mod finalization_host;
mod finalize;
mod kickoff;
mod turn_host;
pub use detached::dispatch;
pub use finalization_host::FinalizationHost;
pub use finalize::finalize;
pub use kickoff::kick_off;
pub use turn_host::{Started, TurnHost};
