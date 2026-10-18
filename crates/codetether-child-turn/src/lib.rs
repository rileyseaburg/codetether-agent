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

mod mailbox_host;
mod message;
mod message_host;
mod message_run;
mod submit;
pub use mailbox_host::MailboxHost;
pub use message::handle_message;
pub use message_host::MessageHost;
pub use message_run::execute as run_message;

mod prompt_host;
mod startup_first;
mod startup_host;
mod startup_message;
pub use prompt_host::PromptHost;
pub use startup_first::begin_first;
pub use startup_host::StartupHost;
pub use startup_message::begin_message;
