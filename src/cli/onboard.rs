//! First-run onboarding flow for new users.

use crate::cli::auth::{SavedCredentials, load_saved_credentials};
use anyhow::Result;
use std::io::{self, Write};

pub const DEFAULT_SERVER: &str = "https://api.codetether.run";

pub async fn ensure_authenticated() -> Result<Option<SavedCredentials>> {
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
        "1" => super::ob_register::register_and_login().await,
        "2" => super::ob_login::login_interactive().await,
        _ => {
            println!("Continuing in local-only mode.\n");
            Ok(None)
        }
    }
}

pub fn prompt_line(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
