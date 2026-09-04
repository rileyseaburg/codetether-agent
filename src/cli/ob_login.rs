//! Interactive login flow for onboarding.

use crate::cli::auth::{
    SavedCredentials, load_saved_credentials, login_with_password, rpassword_prompt,
    write_saved_credentials,
};
use crate::cli::onboard::{DEFAULT_SERVER, prompt_line};
use anyhow::Result;

pub async fn login_interactive() -> Result<Option<SavedCredentials>> {
    let email = prompt_line("Email: ")?;
    if email.is_empty() {
        anyhow::bail!("Email is required");
    }
    let password = rpassword_prompt("Password: ")?;
    if password.is_empty() {
        anyhow::bail!("Password is required");
    }

    println!("Logging in...");
    let client = crate::provider::shared_http::shared_client().clone();
    let login = login_with_password(&client, DEFAULT_SERVER, &email, &password).await?;
    write_saved_credentials(DEFAULT_SERVER, &email, &login)?;
    println!("Logged in as {email}.\n");
    load_saved_credentials()
        .map(Some)
        .ok_or_else(|| anyhow::anyhow!("Failed to load credentials"))
}
