//! Interactive persona relay execution and event presentation.
//!
//! Supply [host::Host] to load providers and [host::View] for presentation.
//! The event payload remains generic so native session snapshots stay outside
//! this crate. Commands and workers share the same state and completion events.
mod binding;
pub mod command;
pub mod events;
pub mod handler;
pub mod host;
pub mod notify;
pub mod persona;
pub mod persona_pick;
pub mod persona_state;
pub mod request;
pub mod run;
pub mod run_step;
pub mod state;
pub mod step_request;
pub mod summary;
pub mod worker;
