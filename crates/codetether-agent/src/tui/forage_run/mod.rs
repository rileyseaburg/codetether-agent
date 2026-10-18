//! TUI `/forage` command integration.
//!
//! Launches OKR-driven forage scans/executions from the chat view and
//! streams results back as system messages.

use codetether_forage_interactive as args;
mod command;
mod drain;
mod host;
pub mod spawn;
pub use codetether_forage_interactive as state;

pub use args::build_tui_forage_args;
pub use command::handle_forage_command;
pub use drain::drain_forage_updates;
pub use state::{ForageState, ForageUpdate};
