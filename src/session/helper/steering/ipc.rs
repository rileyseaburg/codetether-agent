//! Session-addressed local steering transport.

mod client;
mod connection;
mod endpoint;
mod path;
mod server;
mod wire;

pub(super) use client::send;
pub(super) use endpoint::Endpoint;

#[cfg(test)]
mod tests;
