//! Concurrent storage and identity keys for observed remote turns.

use super::types::RemoteTurn;
use dashmap::DashMap;
use std::sync::LazyLock;

pub(super) static TURNS: LazyLock<DashMap<String, RemoteTurn>> = LazyLock::new(DashMap::new);

pub(super) fn key(name: &str, owner_session_id: Option<&str>) -> String {
    format!("{}\0{name}", owner_session_id.unwrap_or_default())
}

pub(super) fn owned(turn: &RemoteTurn, owner_session_id: Option<&str>) -> bool {
    owner_session_id.is_none() || turn.owner_session_id.as_deref() == owner_session_id
}
