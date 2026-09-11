//! Accept pre-multi-session snapshots written by older mux servers.

use std::path::PathBuf;

use serde::Deserialize;

use super::{MuxRuntimeStatus, MuxSession, MuxSnapshot, MuxWindow};
use crate::mux::isolation::Isolation;

/// Flat single-session shape persisted before sessions were nested.
#[derive(Deserialize)]
struct Legacy {
    name: String,
    active_window: u64,
    windows: Vec<MuxWindow>,
    #[serde(default)]
    runtime: Option<MuxRuntimeStatus>,
    #[serde(default)]
    isolation: Isolation,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Wire {
    Current(super::server::Current),
    Legacy(Legacy),
}

impl<'de> Deserialize<'de> for MuxSnapshot {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match Wire::deserialize(deserializer)? {
            Wire::Current(current) => current.into(),
            Wire::Legacy(legacy) => legacy.into(),
        })
    }
}

impl From<Legacy> for MuxSnapshot {
    fn from(legacy: Legacy) -> Self {
        let workspace = legacy
            .windows
            .first()
            .map_or_else(PathBuf::new, |window| window.workspace.clone());
        let session = MuxSession {
            name: legacy.name,
            active_window: legacy.active_window,
            windows: legacy.windows,
            runtime: legacy.runtime,
        };
        Self {
            workspace,
            isolation: legacy.isolation,
            sessions: vec![session],
        }
    }
}
