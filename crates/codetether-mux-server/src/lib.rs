//! Authenticated TCP mux server.
//!
//! [serve] owns one workspace server: listener, authenticated client tasks,
//! session state, PTY and agent processes, workspace leases and persistence.
//! [token] supplies CLI bootstrap credentials shared with server authentication.

mod agent;
mod agent_read;
mod agent_start;
mod client_tasks;
mod connection;
mod context;
mod context_persist;
mod coordination;
mod coordination_identity;
mod coordination_path;
mod dispatch;
mod dispatch_session;
mod mutate;
mod program_operations;
mod program_request;
mod program_scope;
mod program_start;
mod program_steer;
mod program_tail;
mod run;
mod runtime;
mod session_close;
mod session_create;
mod startup;
mod workspace;

pub use run::serve;

#[cfg(test)]
mod tests;

pub mod token;
#[cfg(test)]
mod token_tests;

mod agent_error;
mod client_report;
mod connection_error;
mod program_access;

mod authentication;
