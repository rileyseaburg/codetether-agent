//! Network mux client connection and interactive command loop.

mod attach;
mod connection;
mod exec;
pub(in crate::mux) mod handshake;
pub(super) mod parse;
mod program;
mod proxy;
mod render;
mod session;
mod state;
mod terminal;

pub(super) use attach::attach;
pub(super) use connection::MuxConnection;
pub(super) use handshake::probe;
