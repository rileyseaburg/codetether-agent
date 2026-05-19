//! HTTP request adapters for fetch, XHR, and axios-style commands.

mod send;
mod verbs;

/// Shared HTTP sender.
pub(in crate::browser::session::native) use send::send;
/// Browser-flavored HTTP request handlers.
pub(in crate::browser::session::native) use verbs::{axios, fetch, xhr};
