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
pub(in crate::tool::browserctl) use axios::axios;
/// Network diagnose action.
pub(in crate::tool::browserctl) use diagnose::diagnose;
/// Fetch-style network action.
pub(in crate::tool::browserctl) use fetch::fetch;
/// Network log action.
pub(in crate::tool::browserctl) use log::network_log;
/// Replay network action.
pub(in crate::tool::browserctl) use replay::replay;
/// XHR-style network action.
pub(in crate::tool::browserctl) use xhr::xhr;
