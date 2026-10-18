//! Ordered interactive slash-command dispatch with native execution bindings.
//!
//! Implement [Host], then call [dispatch]. Leading commands consume input;
//! conditional handlers can fall through to simple actions; trailing commands
//! and the legacy unknown-message suppression run last.
mod host;
mod simple;
mod types;
pub use host::Host;
pub use types::{SimpleCommand, TailCommand};
/// Normalize and dispatch a command in the existing interactive priority order.
///
/// # Arguments
/// * `host` — Application command execution bindings.
/// * `command` — Raw command line.
/// # Returns
/// Completes after the applicable operations and status handling.
/// # Examples
/// ```no_run
/// # async fn run(host: &mut impl codetether_slash_command::pipeline::Host) {
/// codetether_slash_command::pipeline::dispatch(host, "/help").await;
/// # }
/// ```
pub async fn dispatch(host: &mut impl Host, command: &str) {
    let normalized = crate::normalize_easy_command(command);
    let normalized = crate::normalize_slash_command(&normalized);
    if let Some((kind, rest)) = crate::routing::leading(&normalized) {
        host.leading(kind, rest).await;
        return;
    }
    if let Some(rest) = crate::command_with_optional_args(&normalized, "/ralph")
        && host.ralph(rest).await
    {
        return;
    }
    if let Some(rest) = crate::command_with_optional_args(&normalized, "/swarm")
        && host.swarm(rest)
    {
        return;
    }
    if host.session(&normalized).await {
        return;
    }
    if let Some(kind) = simple::classify(&normalized) {
        host.simple(kind).await;
    }
    if let Some(rest) = crate::command_with_optional_args(&normalized, "/spawn") {
        let kind = if rest.is_empty() {
            TailCommand::SpawnForm
        } else {
            TailCommand::SpawnTask
        };
        host.tail(kind, rest).await;
        return;
    }
    if let Some(rest) = crate::command_with_optional_args(&normalized, "/kill") {
        host.tail(TailCommand::Kill, rest).await;
        return;
    }
    if crate::command_with_optional_args(&normalized, "/agents").is_some() {
        host.tail(TailCommand::Agents, "").await;
        return;
    }
    if let Some(rest) = crate::command_with_optional_args(&normalized, "/agent") {
        host.tail(TailCommand::Agent, rest).await;
        return;
    }
    if let Some(rest) = crate::command_with_optional_args(&normalized, "/autochat") {
        host.tail(TailCommand::Autochat, rest).await;
        return;
    }
    if !crate::routing::suppress_unknown(&normalized) {
        host.unknown(&normalized);
    }
}
