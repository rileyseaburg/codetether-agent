//! Session-addressed local steering transport.

mod client;
mod connection;
mod endpoint;
mod path;
mod server;
mod wire;

pub(crate) use client::send;
pub use endpoint::Endpoint;

#[cfg(test)]
mod tests;
