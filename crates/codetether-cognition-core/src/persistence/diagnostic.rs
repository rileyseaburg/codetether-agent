//! Load outcomes reported at their original points to the host logger.
use super::PersistedCognitionState;

/// Diagnostic emitted while loading persisted state.
/// The host controls how these events are logged.
///
/// # Examples
/// ```
/// use codetether_cognition_core::persistence::LoadEvent;
/// let event = LoadEvent::TooLarge { size: 200, max: 100 };
/// match event {
///     LoadEvent::TooLarge { size, max } => assert!(size > max),
///     _ => unreachable!(),
/// }
/// ```
pub enum LoadEvent<'a> {
    /// A file exceeded the configured size limit.
    TooLarge { size: u64, max: u64 },
    /// File metadata could not be read.
    StatFailed(&'a std::io::Error),
    /// State text could not be read.
    ReadFailed(&'a std::io::Error),
    /// Persisted and current schema versions differ.
    SchemaMismatch { persisted: u32, current: u32 },
    /// State parsed successfully and has the current schema.
    Loaded(&'a PersistedCognitionState),
    /// State text was not valid for the persisted schema.
    Corrupt(&'a serde_json::Error),
}
