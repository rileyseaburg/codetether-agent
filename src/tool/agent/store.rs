//! Canonical child-thread registry keyed by durable session ID.

use parking_lot::RwLock;
use std::collections::HashMap;

#[path = "store_crud.rs"]
mod crud;
#[path = "store_entry.rs"]
mod entry;
#[path = "store_scope.rs"]
pub(super) mod scope;

pub(super) use crud::{
    contains_name, contains_name_for_owner, get, insert, insert_reserved, remove, update_session,
};
pub(super) use entry::AgentEntry;
pub(super) use scope::{entries_for_parent, get_for_parent, lineage_for_session};

lazy_static::lazy_static! {
    static ref AGENT_STORE: RwLock<HashMap<String, AgentEntry>> = RwLock::new(HashMap::new());
}

#[cfg(test)]
#[path = "store_identity_tests.rs"]
mod identity_tests;
