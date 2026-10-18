/// Runtime control messages.
///
/// `SessionCommand` is sent over the runtime command channel by
/// the frontend handle. Prompt submission transfers ownership of
/// a boxed request to keep channel messages reasonably sized, while
/// cancellation and shutdown commands are lightweight control signals.
/// # Examples
/// ```
/// use codetether_session_runtime::request::SessionCommand;
/// let command: SessionCommand<()> = SessionCommand::CancelCurrent;
/// match command {
///     SessionCommand::CancelCurrent => {},
///     SessionCommand::SubmitPrompt(_) | SessionCommand::Shutdown => unreachable!(),
/// }
/// ```
pub enum SessionCommand<Q> {
    /// Submit a prompt request for execution by the runtime.
    SubmitPrompt(Box<Q>),
    /// Request cancellation of the currently running prompt, if any.
    CancelCurrent,
    /// Request runtime shutdown after notifying any active prompt.
    Shutdown,
}
