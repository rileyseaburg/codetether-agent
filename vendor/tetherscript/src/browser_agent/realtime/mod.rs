//! Deterministic realtime surfaces for agent pages.
//!
//! This module exposes in-memory WebSocket and EventSource state created by the
//! JavaScript host. Agents can inspect opened connections, read outbound
//! WebSocket messages, and inject inbound messages without real network I/O.

mod convert;
mod dispatch;
mod failure;
mod log;
mod model;
mod page;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_failure;
#[cfg(test)]
mod tests_log;

pub use model::*;
