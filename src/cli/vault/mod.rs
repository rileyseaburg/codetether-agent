//! First-class Vault management without a model or initialized provider registry.
//!
//! Use `vault url`, `vault login token|oidc|device`, `vault status`, and `vault logout`.
//! Device login requires an IdP public client and a Vault JWT-type application role.

mod args;
mod device;
mod device_args;
mod login;
mod oidc;
mod settings;
mod status;
mod terminal_guard;
mod token_event;
mod token_input;
mod token_stdin;
pub use args::VaultArgs;
#[cfg(test)]
mod tests;

/// Execute a Vault management command independently of normal provider startup.
///
/// # Arguments
/// * `args` — Parsed Vault command; an omitted action displays status.
/// # Returns
/// Success after displaying status or safely persisting settings.
/// # Errors
/// Returns errors for invalid URLs, failed authentication, unsafe credentials or I/O.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_agent::cli::vault::VaultArgs) -> anyhow::Result<()> {
/// codetether_agent::cli::vault::execute(args).await
/// # }
/// ```
pub async fn execute(args: VaultArgs) -> anyhow::Result<()> {
    match args.action.unwrap_or(args::Action::Status) {
        args::Action::Status => status::status().await,
        args::Action::Url { address } => settings::url(&address),
        args::Action::Logout => settings::logout(),
        args::Action::Login { method } => login::run(method).await,
    }
}
