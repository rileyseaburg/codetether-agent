//! Page navigation lifecycle helpers.

#[path = "navigation/commit_api.rs"]
mod commit_api;
#[path = "navigation/entry.rs"]
mod entry;
#[path = "navigation/history_api.rs"]
mod history_api;
#[path = "navigation/history_list.rs"]
mod history_list;
#[path = "navigation/lifecycle.rs"]
mod lifecycle;
#[path = "navigation/lifecycle_hash.rs"]
mod lifecycle_hash;
#[path = "navigation/lifecycle_js.rs"]
mod lifecycle_js;
#[path = "navigation/lifecycle_log.rs"]
mod lifecycle_log;
#[path = "navigation/load_api.rs"]
mod load_api;
#[path = "navigation/load_methods.rs"]
mod load_methods;
#[path = "navigation/result.rs"]
mod result;
#[path = "navigation/state_api.rs"]
mod state_api;
#[path = "navigation/store.rs"]
mod store;
#[path = "navigation/stored.rs"]
mod stored;
#[path = "navigation/url_kind.rs"]
mod url_kind;

pub(crate) use commit_api::commit_document;
pub use entry::PageHistoryEntry;
pub use result::{NavigationKind, NavigationResult, NavigationStatus};
pub(crate) use store::PageHistory;
