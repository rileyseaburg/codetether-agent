/// Failures in question admission or interaction.
///
/// # Examples
/// ```
/// use codetether_question::Error;
/// assert!(matches!(Error::Dismissed, Error::Dismissed));
/// ```
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The request is malformed or contains ambiguous choices.
    #[error("Invalid question request: {0}")]
    Invalid(String),
    /// This execution has no attached interactive frontend.
    #[error("Questions require an interactive frontend; ask in assistant text instead")]
    Unavailable,
    /// A session may wait for only one question batch at a time.
    #[error("This session already has a pending question")]
    Busy,
    /// The user dismissed the question; no answer should be inferred.
    #[error("The user dismissed this question; no answer was supplied")]
    Dismissed,
}
