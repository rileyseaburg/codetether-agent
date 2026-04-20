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
mod xhr;

pub(in crate::tool::browserctl) use axios::axios;
pub(in crate::tool::browserctl) use diagnose::diagnose;
pub(in crate::tool::browserctl) use fetch::fetch;
pub(in crate::tool::browserctl) use log::network_log;
pub(in crate::tool::browserctl) use xhr::xhr;
