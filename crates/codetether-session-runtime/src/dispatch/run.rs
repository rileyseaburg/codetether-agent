use super::Host;
use crate::request::SessionCommand;
use tokio::sync::mpsc;
/// Consume runtime commands until shutdown or channel closure.
/// # Arguments
/// `host` applies commands; `commands` owns the runtime receiver.
/// # Returns
/// Completes when shutdown is handled or all command senders are dropped.
/// # Examples
/// ```no_run
/// # async fn run<Q: Send>(host: &mut impl codetether_session_runtime::dispatch::Host<Q>, rx: tokio::sync::mpsc::Receiver<codetether_session_runtime::request::SessionCommand<Q>>) {
/// codetether_session_runtime::dispatch::run(host, rx).await;
/// # }
/// ```
pub async fn run<Q: Send>(
    host: &mut impl Host<Q>,
    mut commands: mpsc::Receiver<SessionCommand<Q>>,
) {
    while let Some(command) = commands.recv().await {
        if super::handle(host, command).await {
            break;
        }
    }
}
