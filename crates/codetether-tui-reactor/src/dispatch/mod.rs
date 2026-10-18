//! Event dispatch ordering and post-event work.
//!
//! Implement [Host] with the application's handlers and call [dispatch] using
//! the snapshot taken before waiting and the event returned by [crate::select].
mod after;
mod events;
mod notices;
mod run;
mod state;
mod ticks;
pub use after::After;
pub use events::Events;
pub use notices::Notices;
pub use run::dispatch;
pub use state::State;
pub use ticks::Ticks;
/// Complete native event-dispatch binding.
/// # Implementors
/// Any type implementing all five operation groups.
pub trait Host: State + Events + Notices + Ticks + After {}
impl<T: State + Events + Notices + Ticks + After> Host for T {}
