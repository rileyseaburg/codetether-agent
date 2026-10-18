//! Session object and bounded-load metadata returned to the caller.
/// A loaded session with information about omitted history and source size.
/// # Examples
/// ```
/// let loaded = codetether_session_resume::loading::LoadedSession { session: (), dropped: 2, file_bytes: 100 };
/// assert_eq!(loaded.dropped, 2);
/// ```
pub struct LoadedSession<S> {
    /// Native session object.
    pub session: S,
    /// Number of messages omitted by bounded loading.
    pub dropped: usize,
    /// Original native file size, or zero for archive imports.
    pub file_bytes: u64,
}
