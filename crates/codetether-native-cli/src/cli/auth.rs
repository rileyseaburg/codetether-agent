//! Application identity adapter for shared authentication commands.
use super::AuthArgs;
use anyhow::Result;
pub use codetether_user_auth::{SavedCredentials, load_saved_credentials};

/// Execute authentication using this application's version metadata.
pub async fn execute(args: AuthArgs) -> Result<()> {
    codetether_auth_cli::execute(args, env!("CARGO_PKG_VERSION")).await
}
