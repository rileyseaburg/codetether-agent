//! Owner-only session creation, observation, requests, replies, and stop.
//!
//! The outer router authenticates the owner. Each action owns its body
//! validation and session lock, preserving checks before reading input.
mod create;
mod reply;
mod request;
mod session;
mod stop;
pub(crate) use create::create;
pub(crate) use session::session;
