//! Application logging and adapters for shared cognition persistence.
use codetether_cognition_core::persistence::LoadEvent;
pub use codetether_cognition_core::persistence::PersistedCognitionState;
use std::path::Path;
mod load;
mod path;
mod save;
pub use load::load_state;
pub use save::save_state;
fn log_saved(path: &Path) {
    tracing::debug!(path = %path.display(), "Cognition state persisted");
}
fn log_load(event: LoadEvent<'_>) {
    match event {
        LoadEvent::TooLarge { size, max } => {
            tracing::warn!(
                size_mb = size / (1024 * 1024),
                max_mb = max / (1024 * 1024),
                "Persisted cognition state too large, starting fresh"
            );
        }
        LoadEvent::StatFailed(e) => {
            tracing::warn!(error = %e, "Failed to stat persisted cognition state");
        }
        LoadEvent::ReadFailed(e) => {
            tracing::warn!(error = %e, "Failed to read persisted cognition state");
        }
        LoadEvent::SchemaMismatch { persisted, current } => {
            tracing::warn!(
                persisted_version = persisted,
                current_version = current,
                "Schema version mismatch, starting fresh"
            );
        }
        LoadEvent::Loaded(state) => {
            tracing::info!(
                persisted_at = %state.persisted_at,
                personas = state.personas.len(),
                beliefs = state.beliefs.len(),
                "Loaded persisted cognition state"
            );
        }
        LoadEvent::Corrupt(e) => {
            tracing::warn!(error = %e, "Corrupt persisted cognition state, starting fresh");
        }
    }
}
