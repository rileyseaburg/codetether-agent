//! Final-message decision without changing durable goal completion.

/// Reason an execution loop should run another model step.
/// # Examples
/// ```rust
/// use codetether_session_orchestration::Continuation;
/// let reason = Continuation::Steering;
/// match reason {
///     Continuation::Steering => {},
///     Continuation::Goal => {},
///     Continuation::Finish => {},
/// }
/// ```
pub enum Continuation {
    /// A human correction arrived before the inbox closed.
    Steering,
    /// The existing durable goal is still active.
    Goal,
    /// No accepted steering or active goal requires further execution.
    Finish,
}
