use super::Host;
use crate::request::SessionCommand;
/// Dispatch a single runtime command.
/// # Arguments
/// `command` transfers a request or carries a control signal; `host` applies it.
/// # Returns
/// True only when shutdown requests termination of the command loop.
/// # Examples
/// ```no_run
/// # async fn cancel<Q: Send>(host: &mut impl codetether_session_runtime::dispatch::Host<Q>) {
/// assert!(!codetether_session_runtime::dispatch::handle(host, codetether_session_runtime::request::SessionCommand::CancelCurrent).await);
/// # }
/// ```
pub async fn handle<Q: Send>(host: &mut impl Host<Q>, command: SessionCommand<Q>) -> bool {
    match command {
        SessionCommand::SubmitPrompt(request) => super::submit(host, *request).await,
        SessionCommand::CancelCurrent => notify_cancel(host, false),
        SessionCommand::Shutdown => notify_cancel(host, true),
    }
}
fn notify_cancel<Q>(host: &impl Host<Q>, should_break: bool) -> bool {
    host.cancel();
    should_break
}
