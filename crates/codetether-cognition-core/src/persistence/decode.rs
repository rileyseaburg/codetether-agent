//! Decode persisted JSON and enforce the current schema version.
use super::{LoadEvent, PersistedCognitionState, state::SCHEMA_VERSION};

pub(super) fn decode(
    data: &str,
    report: &mut impl FnMut(LoadEvent<'_>),
) -> Option<PersistedCognitionState> {
    match serde_json::from_str::<PersistedCognitionState>(&data) {
        Ok(state) => {
            if state.schema_version != SCHEMA_VERSION {
                report(LoadEvent::SchemaMismatch {
                    persisted: state.schema_version,
                    current: SCHEMA_VERSION,
                });
                return None;
            }
            report(LoadEvent::Loaded(&state));
            Some(state)
        }
        Err(e) => {
            report(LoadEvent::Corrupt(&e));
            None
        }
    }
}
