//! Permission and geolocation emulation for agent-controlled pages.
//!
//! The module keeps deterministic, origin-scoped permission state separate
//! from real host capabilities. It is intentionally metadata-first so tests
//! and agents can emulate browser decisions without touching the OS.

mod context;
mod context_apply;
mod geolocation;
mod grant;
mod kind;
mod page;
mod page_store;
mod position;
mod runtime;
mod script;
mod state;
mod store;
mod store_read;
mod store_write;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_notification;
#[cfg(test)]
mod tests_persistence;
#[cfg(test)]
mod tests_runtime;

pub use geolocation::{GeolocationEmulation, GeolocationError, GeolocationErrorCode};
pub use grant::PermissionGrant;
pub use kind::BrowserPermission;
pub use position::GeolocationPosition;
pub use state::PermissionState;
pub use store::PermissionStore;
