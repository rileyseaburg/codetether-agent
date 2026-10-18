//! View-level Enter routing for chat, selectors and detail panes.
//!
//! [dispatch] selects handlers through [Host]; native code retains session
//! ownership and the concrete submission/settings implementations.
mod binding;
mod host;
mod run;
pub use host::{Async, Host, Sync};
pub use run::dispatch;
