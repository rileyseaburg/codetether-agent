//! Convert legacy single-session snapshots to the current model.
use super::*;

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
