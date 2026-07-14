//! Attached-client tracking of the latest server snapshot.

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::mux::model::MuxSnapshot;
use crate::mux::protocol::ServerResponse;

pub(super) fn update(current: &mut Option<MuxSnapshot>, response: &ServerResponse) {
    if let ServerResponse::Snapshot { state } = response {
        *current = Some(state.clone());
    }
}

pub(super) fn workspace(state: &Option<MuxSnapshot>) -> Result<&Path> {
    let state = state
        .as_ref()
        .ok_or_else(|| anyhow!("mux state is unavailable"))?;
    state
        .windows
        .iter()
        .find(|window| window.id == state.active_window)
        .map(|window| window.workspace.as_path())
        .ok_or_else(|| anyhow!("active mux window is unavailable"))
}
