//! Isolated browser context containing agent pages.

#[path = "context_state.rs"]
pub mod context_state;
#[path = "context_state_page.rs"]
mod context_state_page;
#[path = "context/core.rs"]
mod core;
#[path = "indexed_db/mod.rs"]
pub(crate) mod indexed_db;
#[path = "persistence/mod.rs"]
pub(crate) mod persistence;
#[path = "context/service_worker/mod.rs"]
pub(crate) mod service_worker;
#[path = "context/storage/mod.rs"]
mod storage;

use self::context_state::SharedContextState;
use crate::browser_agent::page::BrowserPage;

/// A collection of pages sharing one automation context.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowserContext {
    pub(crate) pages: Vec<BrowserPage>,
    state: SharedContextState,
    pub(crate) security_policy: super::exports::SecurityPolicy,
    pub(crate) permissions: super::permissions::PermissionStore,
    pub(crate) geolocation: super::permissions::GeolocationEmulation,
}
