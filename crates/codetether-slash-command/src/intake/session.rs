use std::future::Future;
/// Command operations over one retained native session borrow.
/// # Implementors
/// Native adapters borrow application and session fields for the whole dispatch.
pub trait SessionHost {
    /// Run compatibility/parity commands, returning whether one consumed input.
    fn parity(&mut self, prompt: &str) -> impl Future<Output = bool>;
    /// Run the ordinary slash-command handler.
    fn command(&mut self, prompt: &str) -> impl Future<Output = ()>;
    /// Clear input after ordinary command handling.
    fn clear_input(&mut self);
}
/// Dispatch commands while one mutable session borrow is retained.
/// # Arguments
/// Host retains the session and native command services; prompt is slash input.
/// # Returns
/// Completes after parity consumption or ordinary dispatch and input clearing.
/// # Examples
/// ```ignore
/// with_session(&mut session_host, "/help").await;
/// ```
pub async fn with_session<H: SessionHost>(host: &mut H, prompt: &str) {
    if host.parity(prompt).await {
        return;
    }
    host.command(prompt).await;
    host.clear_input();
}
