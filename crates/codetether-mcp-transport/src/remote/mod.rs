//! Protocol operations on a shared RPC connection.
//!
//! These methods encode requests and decode responses. Callers retain access
//! checks, identity choice, server caches and initialization notifications.
mod initialize;
mod prompts;
mod resources;
mod tools;
