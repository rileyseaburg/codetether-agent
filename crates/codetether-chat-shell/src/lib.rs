//! Background execution for chat commands prefixed with an exclamation mark.
//!
//! [State] owns the in-flight command and its result channel. Implement [View]
//! to bind transcript and input updates, then call [submit] and [drain].
mod command;
mod drain;
mod event;
mod frontend;
mod run;
mod state;
mod truncate;
mod view;
pub use command::submit;
pub use drain::drain;
pub use event::ShellEvent;
pub use state::State;
pub use view::View;
