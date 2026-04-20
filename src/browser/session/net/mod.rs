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

pub(crate) mod axios;
mod axios_tmpl;
pub(crate) mod diagnose;
mod diagnose_tmpl;
mod fallback_js;
pub(crate) mod fetch;
mod fetch_tmpl;
pub(crate) mod log;
pub(crate) mod xhr;
mod xhr_tmpl;

