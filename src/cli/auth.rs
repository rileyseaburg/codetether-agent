//! Application identity adapter for shared authentication commands.
use super::AuthArgs;
use anyhow::Result;
pub(crate) use codetether_user_auth::{LoginResponsePayload, write_saved_credentials};
pub use codetether_user_auth::{SavedCredentials, load_saved_credentials};
pub(crate) use codetether_user_auth_cli::rpassword_prompt;
use reqwest::Client;

/// Execute authentication using this application's version metadata.
pub async fn execute(args: AuthArgs) -> Result<()> {
    codetether_auth_cli::execute(args, env!("CARGO_PKG_VERSION")).await
}

pub(crate) async fn login_with_password(
    client: &Client,
    server_url: &str,
    email: &str,
    password: &str,
) -> Result<LoginResponsePayload> {
    codetether_user_auth::login_with_password(
        client,
        server_url,
        email,
        password,
        env!("CARGO_PKG_VERSION"),
    )
    .await
}
