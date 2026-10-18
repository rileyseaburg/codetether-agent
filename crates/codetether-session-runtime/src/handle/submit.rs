use super::{Control, SessionHandle};
use crate::request::SessionCommand;
impl<Q, C: Control> SessionHandle<Q, C> {
    /// Submit one moved-session prompt.
    ///
    /// The request is boxed into a [`SessionCommand::SubmitPrompt`] and sent to
    /// the runtime. The runtime may still reject the request later if it is busy;
    /// this method only reports whether the command channel accepted ownership
    /// of the request.
    ///
    /// # Arguments
    ///
    /// * `request` - Prompt request to transfer to the runtime task.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the command was queued. Returns the original
    /// request when the runtime channel is closed before accepting it.
    ///
    /// # Errors
    ///
    /// The error value is the unsent prompt request, allowing the caller to keep
    /// or retry the prompt without reconstructing it.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn queue<Q, C: codetether_session_runtime::handle::Control>(handle: &codetether_session_runtime::handle::SessionHandle<Q, C>, request: Q) {
    /// let _result = handle.submit(request).await;
    /// # }
    /// ```
    pub async fn submit(&self, request: Q) -> Result<(), Q> {
        let command = SessionCommand::SubmitPrompt(Box::new(request));
        match self.tx.send(command).await {
            Ok(()) => Ok(()),
            Err(err) => match err.0 {
                SessionCommand::SubmitPrompt(request) => Err(*request),
                SessionCommand::CancelCurrent | SessionCommand::Shutdown => unreachable!(),
            },
        }
    }
}
