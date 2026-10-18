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
