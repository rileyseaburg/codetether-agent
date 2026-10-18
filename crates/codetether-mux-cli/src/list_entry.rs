//! Serialized CLI listing entry.
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct ListedSession {
    pub(super) name: String,
    pub(super) workspace: String,
    pub(super) address: String,
    pub(super) pid: u32,
    pub(super) windows: usize,
    pub(super) active: u64,
    pub(super) reachable: bool,
}
