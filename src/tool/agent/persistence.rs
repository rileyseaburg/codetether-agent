//! Durable spawned-agent manifests and recovery orchestration.

#[path = "persistence/atomic.rs"]
pub(crate) mod atomic;
#[path = "persistence/hydrate.rs"]
mod hydrate;
#[path = "persistence/lifecycle.rs"]
mod lifecycle;
#[path = "persistence/lookup.rs"]
mod lookup;
#[path = "persistence/manifest.rs"]
mod manifest;
#[path = "persistence/manifest_io.rs"]
mod manifest_io;
#[path = "persistence/manifest_scan.rs"]
mod manifest_scan;
#[path = "persistence/paths.rs"]
pub(crate) mod paths;
#[path = "persistence/restore.rs"]
mod restore;
#[path = "persistence/tree_target.rs"]
mod tree_target;

pub(crate) use hydrate::parent as hydrate_parent;
pub(in crate::tool::agent) use lifecycle::{activate_resume, close, exists, prepare_resume};
#[cfg(test)]
pub(in crate::tool::agent) use lookup::is_open;
pub(in crate::tool::agent) use lookup::{child_ids, durable_id};
pub(in crate::tool::agent) use manifest_io::{remove, save};

pub(crate) async fn hydrate_best_effort(parent: &str) {
    if let Err(error) = hydrate_parent(Some(parent)).await {
        tracing::warn!(parent_session_id = parent, %error, "Agent recovery failed");
    }
}

#[cfg(test)]
#[path = "persistence/close_receipt_tests.rs"]
mod close_receipt_tests;
#[cfg(test)]
#[path = "persistence/lifecycle_tests.rs"]
mod lifecycle_tests;
#[cfg(test)]
#[path = "persistence/restart_tests.rs"]
mod restart_tests;
#[cfg(test)]
#[path = "persistence/restart_test_support.rs"]
pub(in crate::tool::agent) mod test_support;
