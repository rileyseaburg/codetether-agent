//! register command orchestration.
use crate::RegisterAuthArgs;
use anyhow::Result;
use codetether_user_auth::{login_with_password, write_saved_credentials};
use reqwest::Client;

/// Register an account, then log in and save its credentials.
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
/// # async fn example(args: codetether_user_auth_cli::RegisterAuthArgs) -> anyhow::Result<()> {
/// codetether_user_auth_cli::register(args, "1.0").await
/// # }
/// ```
pub async fn execute(args: RegisterAuthArgs, application_version: &str) -> Result<()> {
    let server_url = args.server.trim_end_matches('/').to_string();

    let email = crate::email::read(args.email)?;

    let password = crate::registration_password::read()?;

    println!("Registering with {}...", server_url);

    let client = Client::new();
    let reg = crate::register_request::send(
        &client,
        &server_url,
        &email,
        &password,
        &args.first_name,
        &args.last_name,
        &args.referral_source,
        application_version,
    )
    .await?;

    crate::register_report::print(&reg);

    // Auto-login and save credentials for the worker.
    println!("Logging in...");
    let login = login_with_password(
        &client,
        &server_url,
        &reg.email,
        &password,
        application_version,
    )
    .await?;
    let cred_path = write_saved_credentials(&server_url, &reg.email, &login)?;

    crate::login_report::print(&login, &reg.email, &cred_path);

    Ok(())
}
