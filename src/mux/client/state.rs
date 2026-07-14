//! Attached-client tracking of the latest server snapshot.

use anyhow::{Result, anyhow};
use std::path::PathBuf;

use crate::mux::model::MuxSnapshot;
use crate::mux::protocol::ServerResponse;

pub(super) fn update(current: &mut Option<MuxSnapshot>, response: &ServerResponse) {
    if let ServerResponse::Snapshot { state } = response {
        *current = Some(state.clone());
    }
}

pub(super) fn active_id(state: &Option<MuxSnapshot>) -> Result<u64> {
    state
        .as_ref()
        .map(|value| value.active_window)
        .ok_or_else(|| anyhow!("mux state is unavailable"))
}

pub(super) fn active_workspace(state: &Option<MuxSnapshot>) -> Result<PathBuf> {
    let snapshot = state
        .as_ref()
        .ok_or_else(|| anyhow!("mux state is unavailable"))?;
    snapshot
        .windows
        .iter()
        .find(|window| window.id == snapshot.active_window)
        .map(|window| window.workspace.clone())
        .ok_or_else(|| anyhow!("active mux window is unavailable"))
}
