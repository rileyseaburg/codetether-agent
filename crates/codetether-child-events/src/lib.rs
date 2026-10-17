//! Child event collection and cancellation-safe task ownership.
//!
//! [Host] observes and maps native session events into [Event] values.
//! [collect_events] enforces an inactivity deadline and accumulates output;
//! [run] also joins or aborts the [ChildTask] and returns the updated session.
mod collect;
mod event;
mod finalize;
mod handle;
mod host;
mod run;
mod state;
mod task;
mod text;
pub use collect::{collect_events, collect_events_with_idle_timeout};
pub use event::Event;
pub use host::Host;
pub use run::run;
pub use state::EventLoopState;
pub use task::ChildTask;
pub use text::truncate_preview;
