//! Persistence diagnostics using the original application tracing target.
use codetether_cognition_core::persistence::LoadEvent;
use std::path::Path;
pub(super) fn log_saved(path: &Path) {
    tracing::debug!(target: "codetether_agent::cognition::persistence", path = %path.display(), "Cognition state persisted");
}
pub(super) fn log_load(event: LoadEvent<'_>) {
    match event {
        LoadEvent::TooLarge { size, max } => {
            tracing::warn!(target: "codetether_agent::cognition::persistence",
                size_mb = size / (1024 * 1024),
                max_mb = max / (1024 * 1024),
                "Persisted cognition state too large, starting fresh"
            );
        }
        LoadEvent::StatFailed(e) => {
            tracing::warn!(target: "codetether_agent::cognition::persistence", error = %e, "Failed to stat persisted cognition state");
        }
        LoadEvent::ReadFailed(e) => {
            tracing::warn!(target: "codetether_agent::cognition::persistence", error = %e, "Failed to read persisted cognition state");
        }
        LoadEvent::SchemaMismatch { persisted, current } => {
            tracing::warn!(target: "codetether_agent::cognition::persistence",
                persisted_version = persisted,
                current_version = current,
                "Schema version mismatch, starting fresh"
            );
        }
        LoadEvent::Loaded(state) => {
            tracing::info!(target: "codetether_agent::cognition::persistence",
                persisted_at = %state.persisted_at,
                personas = state.personas.len(),
                beliefs = state.beliefs.len(),
                "Loaded persisted cognition state"
            );
        }
        LoadEvent::Corrupt(e) => {
            tracing::warn!(target: "codetether_agent::cognition::persistence", error = %e, "Corrupt persisted cognition state, starting fresh");
        }
    }
}
