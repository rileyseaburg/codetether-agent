//! Authentication dispatch responsibilities.
use crate::{AuthArgs, AuthCommand, authenticate_codex, authenticate_copilot};
use anyhow::Result;

/// Execute an account or provider authentication command.
///
/// # Arguments
/// * `args` - Selected authentication operation.
/// * `application_version` - Caller version included in outgoing user agents.
/// # Returns
/// Unit after the command completes and reports its outcome.
/// # Errors
/// Returns invalid-input, authentication, transport or credential-store errors.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_auth_cli::AuthArgs) -> anyhow::Result<()> {
/// codetether_auth_cli::execute(args, "1.0").await
/// # }
/// ```
pub async fn execute(args: AuthArgs, application_version: &str) -> Result<()> {
    match args.command {
        AuthCommand::Copilot(copilot_args) => {
            authenticate_copilot(copilot_args, application_version).await
        }
        AuthCommand::Bedrock(bedrock_args) => {
            codetether_bedrock_auth_cli::execute(bedrock_args).await
        }
        AuthCommand::Codex(codex_args) => authenticate_codex(codex_args, application_version).await,
        AuthCommand::Cookies(cookie_args) => codetether_cookie_auth_cli::execute(cookie_args).await,
        AuthCommand::Register(register_args) => {
            codetether_user_auth_cli::register(register_args, application_version).await
        }
        AuthCommand::Login(login_args) => {
            codetether_user_auth_cli::login(login_args, application_version).await
        }
    }
}
