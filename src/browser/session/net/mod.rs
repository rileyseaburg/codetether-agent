//! # Network-log inspection and in-page HTTP replay.
//!
//! Each handler runs JS inside the active tab so requests inherit the
//! real browser's cookies, Origin, TLS fingerprint, and service-worker
//! routing. This lets the agent skip driving React forms and replay
//! the exact backend call.
//!
//! ## Modules
//!
//! - [`log`] — read captured `__codetether_net_log` entries
//! - [`fetch`] — replay via page `fetch()`
//! - [`xhr`] — replay via raw `XMLHttpRequest`
//! - [`axios`] — replay via page's axios instance
//! - [`diagnose`] — dump service workers, CSP, axios instances
//! - [`fallback_js`] — shared JS for header inheritance + fallback hints

mod axios;
mod axios_tmpl;
mod diagnose;
mod diagnose_tmpl;
mod fallback_js;
mod fetch;
mod fetch_tmpl;
mod log;
mod xhr;
mod xhr_tmpl;

pub(super) use axios::axios;
pub(super) use diagnose::diagnose;
pub(super) use fetch::fetch;
pub(super) use log::network_log;
pub(super) use xhr::xhr;
