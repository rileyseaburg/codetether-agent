//! Authenticated mux client transport and local snapshot tracking.
//!
//! [MuxConnection] authenticates a session or server binding and exchanges
//! shared protocol messages. [handshake::probe] tolerates reported versions for
//! discovery; normal connections enforce the supported protocol version.
mod connect;
mod connection;
mod connection_version;
pub mod handshake;
mod request;
mod state;
pub use connection::MuxConnection;
pub use state::ClientState;

mod command;
mod parse;
mod parse_args;
pub mod render;
mod render_snapshot;
pub use command::ParsedCommand;
pub use parse::parse;
#[cfg(test)]
mod exec_tests;
#[cfg(test)]
mod parse_tests;
