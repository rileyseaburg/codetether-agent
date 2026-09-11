//! Authenticated TCP mux server.

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

pub(super) use run::serve;

#[cfg(test)]
mod tests;
