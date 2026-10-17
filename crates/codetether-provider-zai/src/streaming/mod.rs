//! Streaming HTTP execution, SSE state, and caller-specific usage accounting.
//!
//! The application entry point retains total prompt counts; injected-client
//! streams keep their existing cache-subtracted usage interpretation.
mod application;
mod buffer;
mod choice;
mod decode;
mod error;
mod frame;
mod injected;
mod send;
mod state;
mod transport;
mod usage;
pub use application::complete_application;
#[cfg(test)]
mod tests;
