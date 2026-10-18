//! login command orchestration.
use crate::{LoginAuthArgs, password::rpassword_prompt};
use anyhow::Result;
use codetether_user_auth::{login_with_password, write_saved_credentials};
use reqwest::Client;

/// Log in with email/password and save the returned credentials.
///
/// # Arguments
/// * `args` - Server and optional account details.
/// * `application_version` - Version used in the server request User-Agent.
/// # Returns
/// Unit after saving credentials and printing completion details.
/// # Errors
/// Returns input, HTTP, response-decoding or credential persistence errors.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_user_auth_cli::LoginAuthArgs) -> anyhow::Result<()> {
/// codetether_user_auth_cli::login(args, "1.0").await
/// # }
/// ```
pub async fn execute(args: LoginAuthArgs, application_version: &str) -> Result<()> {
    let server_url = args.server.trim_end_matches('/').to_string();

    // Prompt for email if not provided
    let email = crate::email::read(args.email)?;

    // Prompt for password (no echo)
    let password = rpassword_prompt("Password: ")?;
    if password.is_empty() {
        anyhow::bail!("Password is required");
    }

    println!("Authenticating with {}...", server_url);

    let client = Client::new();

    let login =
        login_with_password(&client, &server_url, &email, &password, application_version).await?;
    let cred_path = write_saved_credentials(&server_url, &email, &login)?;

    crate::login_report::print(&login, &email, &cred_path);

    Ok(())
}
