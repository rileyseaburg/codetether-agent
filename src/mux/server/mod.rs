//! Authenticated TCP mux server.

mod connection;
mod context;
mod dispatch;
mod mutate;
mod run;

pub(super) use run::serve;

#[cfg(test)]
mod tests;
