//! First-run onboarding flow for new users.

use crate::prompt_line;
use anyhow::Result;
use codetether_user_auth::{SavedCredentials, load_saved_credentials};

/// Offer account creation, login or local-only operation when credentials are absent.
/// # Arguments
/// * `version` - Calling application's version sent during password login.
/// # Returns
/// Existing/new credentials, or None when onboarding is skipped.
/// # Errors
/// Returns prompt, registration, login or credential-persistence errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _credentials = codetether_onboarding::ensure_authenticated("1.0").await?;
/// # Ok(()) }
/// ```
pub async fn ensure_authenticated(version: &str) -> Result<Option<SavedCredentials>> {
    if let Some(creds) = load_saved_credentials() {
        return Ok(Some(creds));
    }

    println!("\nWelcome to CodeTether! 🚀\n");
    println!("It looks like you haven't set up an account yet.");
    println!("CodeTether is free to get started — no credit card required.\n");
    println!("  [1] Create a free account");
    println!("  [2] Log in to existing account");
    println!("  [3] Skip for now (local-only mode)\n");

    let choice = prompt_line("Choice: ")?;
    match choice.trim() {
        "1" => crate::register_and_login(version).await,
        "2" => crate::login_interactive(version).await,
        _ => {
            println!("Continuing in local-only mode.\n");
            Ok(None)
        }
    }
}
