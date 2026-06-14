//! CLI arguments for `codetether auth bedrock`.

use super::LoginMode;
use crate::provider::bedrock::token_gen;
use clap::Parser;

/// Arguments for `codetether auth bedrock`.
#[derive(Parser, Debug)]
pub struct BedrockAuthArgs {
    /// AWS SSO portal URL; auto-selects a matching AWS CLI profile
    #[arg(long)]
    pub sso: Option<String>,
    /// AWS profile to use (SSO profile recommended)
    #[arg(long, env = "AWS_PROFILE")]
    pub profile: Option<String>,
    /// AWS region to sign the token for
    #[arg(long, env = "AWS_REGION")]
    pub region: Option<String>,
    /// Force fresh SSO login using device-code flow
    #[arg(long, default_value_t = false)]
    pub device_code: bool,
    /// Force fresh SSO login using browser redirect flow
    #[arg(long, default_value_t = false)]
    pub browser: bool,
    /// Never start SSO login; use only existing credentials
    #[arg(long, default_value_t = false)]
    pub no_login: bool,
    /// Token lifetime in seconds (max 43200 = 12h)
    #[arg(long, default_value_t = token_gen::DEFAULT_EXPIRES_SECS)]
    pub expires_secs: u64,
    /// Store the token in HashiCorp Vault as the `bedrock` provider key
    #[arg(long, default_value_t = false)]
    pub save: bool,
    /// Print only the raw token (for command substitution / scripts)
    #[arg(long, default_value_t = false)]
    pub raw: bool,
    /// Save without printing the token; requires --save
    #[arg(long, default_value_t = false)]
    pub save_only: bool,
    /// Skip live validation against the Bedrock management API
    #[arg(long, default_value_t = false)]
    pub no_validate: bool,
    /// Deprecated compatibility knob; use --device-code, --browser, or --no-login
    #[arg(long, value_enum, default_value_t = LoginMode::Auto, hide = true)]
    pub login: LoginMode,
}
