//! Authenticated TCP mux server.

mod connection;
mod context;
mod dispatch;
mod mutate;
mod program;
mod program_operations;
mod program_request;
mod program_start;
mod run;

pub(super) use run::serve;

#[cfg(test)]
mod tests;
