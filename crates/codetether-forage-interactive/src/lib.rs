//! Interactive forage lifecycle: defaults, launch, progress messages and run state.
//!
//! Implement [Host] for application execution and [View] for the UI, then call
//! [handle_forage_command]. Drain [ForageState] to present background updates.
mod args;
mod command;
mod host;
mod spawn;
mod state;
pub use args::build_tui_forage_args;
pub use command::handle_forage_command;
pub use host::{Host, View};
pub use spawn::spawn_forage_run;
pub use state::{ForageState, ForageUpdate};
