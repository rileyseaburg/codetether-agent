//! Guard UUID links before persisting final relay outcomes.
use uuid::Uuid;

/// Guarded UUID parse that logs warnings on invalid input instead of returning NIL UUID.
/// Returns None for invalid UUIDs, allowing callers to skip operations rather than corrupt data.
pub(super) fn parse_uuid_guarded(s: &str, context: &str) -> Option<Uuid> {
    match s.parse::<Uuid>() {
        Ok(uuid) => Some(uuid),
        Err(e) => {
            tracing::warn!(
                context,
                uuid_str = %s,
                error = %e,
                "Invalid UUID string - skipping operation"
            );
            None
        }
    }
}
