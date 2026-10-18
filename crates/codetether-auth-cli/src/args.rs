//! Authentication args responsibilities.
use crate::{CodexAuthArgs, CookieAuthArgs, CopilotAuthArgs, LoginAuthArgs, RegisterAuthArgs};
use clap::{Parser, Subcommand};

/// Parsed authentication command.
///
/// # Examples
/// ```
/// use clap::Parser;
/// let args = codetether_auth_cli::AuthArgs::parse_from(["auth", "codex", "--device-code"]);
/// assert!(matches!(args.command, codetether_auth_cli::AuthCommand::Codex(_)));
/// ```
#[derive(Parser, Debug)]
pub struct AuthArgs {
    /// Authentication operation to execute.
    #[command(subcommand)]
    pub command: AuthCommand,
}

/// Account and provider authentication operations.
///
/// # Examples
/// ```
/// let command = codetether_auth_cli::AuthCommand::Codex(codetether_auth_cli::CodexAuthArgs { device_code: true });
/// assert!(matches!(command, codetether_auth_cli::AuthCommand::Codex(_)));
/// ```
#[derive(Subcommand, Debug)]
pub enum AuthCommand {
    /// Authenticate with GitHub Copilot using device flow
    Copilot(CopilotAuthArgs),

    /// Mint a short-term Amazon Bedrock API key from AWS credentials (supports SSO/IdP profiles)
    Bedrock(codetether_bedrock_auth_cli::BedrockAuthArgs),

    /// Authenticate OpenAI Codex with a ChatGPT subscription via OAuth (Plus/Pro/Team/Enterprise)
    Codex(CodexAuthArgs),

    /// Import browser cookies (Netscape format) into Vault for a provider
    Cookies(CookieAuthArgs),

    /// Register a new CodeTether account with email/password
    Register(RegisterAuthArgs),

    /// Login to a CodeTether server with email/password
    Login(LoginAuthArgs),
}
