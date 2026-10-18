//! Interactive login flow for onboarding.

use crate::{DEFAULT_SERVER, prompt_line};
use anyhow::Result;
use codetether_user_auth::{
    SavedCredentials, load_saved_credentials, login_with_password, write_saved_credentials,
};
use codetether_user_auth_cli::rpassword_prompt;
use reqwest::Client;

/// Run interactive password login.
/// # Arguments
/// * `version` - Calling application's version sent during login.
/// # Returns
/// Credentials reloaded from the shared local cache after successful login.
/// # Errors
/// Returns validation, prompt, network, authentication or persistence errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _credentials = codetether_onboarding::login_interactive("1.0").await?;
/// # Ok(()) }
/// ```
pub async fn login_interactive(version: &str) -> Result<Option<SavedCredentials>> {
    let email = prompt_line("Email: ")?;
    if email.is_empty() {
        anyhow::bail!("Email is required");
    }
    let password = rpassword_prompt("Password: ")?;
    if password.is_empty() {
        anyhow::bail!("Password is required");
    }

    println!("Logging in...");
    let client = Client::new();
    let login = login_with_password(&client, DEFAULT_SERVER, &email, &password, version).await?;
    write_saved_credentials(DEFAULT_SERVER, &email, &login)?;
    println!("Logged in as {email}.\n");
    load_saved_credentials()
        .map(Some)
        .ok_or_else(|| anyhow::anyhow!("Failed to load credentials"))
}
