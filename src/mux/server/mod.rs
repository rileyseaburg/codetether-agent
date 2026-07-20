//! Authenticated TCP mux server.

mod agent;
mod agent_read;
mod agent_start;
mod connection;
mod context;
mod context_persist;
mod coordination;
mod coordination_identity;
mod coordination_path;
mod dispatch;
mod mutate;
mod program;
mod program_operations;
mod program_request;
mod program_start;
mod program_tail;
mod run;
mod startup;

pub(super) use run::serve;

#[cfg(test)]
mod tests;
