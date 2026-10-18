//! Interactive attachment to a server-owned mux terminal.
//!
//! [run] forwards locally filtered input and ordered remote output while its
//! terminal guard owns raw mode, replay synchronization and restoration.
//! The caller supplies clipboard access from the client environment.
mod outcome;
mod output;
mod presentation;
mod run;
mod terminal;
pub use outcome::Outcome;
pub use run::run;

mod output_event;

mod control;
mod dispatch;
pub mod program;
mod program_attach;
mod program_dispatch;
mod program_request;
mod program_start;
pub use control::detach;
pub use dispatch::execute as execute_command;
