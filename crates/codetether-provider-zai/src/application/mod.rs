//! Application-compatible Z.AI provider profile.
//!
//! This profile retains unmodified endpoint strings, newline history joining,
//! first-only packed tool replies, and total-input SSE accounting. The crate
//! root provider keeps its existing injected-client conventions.
mod capability;
mod catalog;
mod complete;
mod construct;
mod debug;
mod messages;
#[cfg(test)]
mod preview;
mod provider_impl;
mod state;
mod stream;
mod tools;
pub use state::ZaiProvider;
#[cfg(test)]
mod tests;
