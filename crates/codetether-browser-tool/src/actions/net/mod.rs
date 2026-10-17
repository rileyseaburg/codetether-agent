//! # Network inspection & replay browserctl actions.
//!
//! Thin wrappers that translate [`BrowserCtlInput`] fields into typed
//! request structs and delegate to [`BrowserCommand`] dispatch.
//!
//! ## Modules
//!
//! - [`log`] — `network_log` action
//! - [`fetch`] — in-page `fetch()` replay
//! - [`xhr`] — raw XMLHttpRequest replay
//! - [`axios`] — page axios instance replay
//! - [`diagnose`] — dump page HTTP plumbing

mod axios;
mod diagnose;
mod fetch;
mod log;
mod replay;
mod xhr;

/// Axios-style network action.
pub(crate) use axios::axios;
/// Network diagnose action.
pub(crate) use diagnose::diagnose;
/// Fetch-style network action.
pub(crate) use fetch::fetch;
/// Network log action.
pub(crate) use log::network_log;
/// Replay network action.
pub(crate) use replay::replay;
/// XHR-style network action.
pub(crate) use xhr::xhr;
