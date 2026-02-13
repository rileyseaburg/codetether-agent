//! Provider authentication commands.

use super::{AuthArgs, AuthCommand, CopilotAuthArgs, LoginAuthArgs, RegisterAuthArgs};
use crate::provider::copilot::normalize_enterprise_domain;
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::time::{Duration, sleep};

const DEFAULT_GITHUB_DOMAIN: &str = "github.com";
const OAUTH_POLLING_SAFETY_MARGIN_MS: u64 = 3000;

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    #[serde(default)]
    verification_uri_complete: Option<String>,
    #[serde(default)]
    interval: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    #[serde(default)]
    access_token: Option<String>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
    #[serde(default)]
    interval: Option<u64>,
}

pub async fn execute(args: AuthArgs) -> Result<()> {
    match args.command {
        AuthCommand::Copilot(copilot_args) => authenticate_copilot(copilot_args).await,
        AuthCommand::Register(register_args) => authenticate_register(register_args).await,
        AuthCommand::Login(login_args) => authenticate_login(login_args).await,
    }
}

#[derive(Debug, Deserialize)]
struct LoginResponsePayload {
    access_token: String,
    expires_at: String,
    user: serde_json::Value,
}

async fn login_with_password(
    client: &Client,
    server_url: &str,
    email: &str,
    password: &str,
) -> Result<LoginResponsePayload> {
    let user_agent = format!("codetether-agent/{}", env!("CARGO_PKG_VERSION"));

    let resp = client
        .post(format!("{}/v1/users/login", server_url))
        .header("User-Agent", &user_agent)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password,
        }))
        .send()
        .await
        .context("Failed to connect to CodeTether server")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        let detail = body
            .get("detail")
            .and_then(|v| v.as_str())
            .unwrap_or("Authentication failed");
        anyhow::bail!("Login failed ({}): {}", status, detail);
    }

    let login: LoginResponsePayload = resp
        .json()
        .await
        .context("Failed to parse login response")?;

    Ok(login)
}

fn write_saved_credentials(
    server_url: &str,
    email: &str,
    login: &LoginResponsePayload,
) -> Result<PathBuf> {
    // Store token to ~/.config/codetether-agent/credentials.json
    let cred_path = credential_file_path()?;
    if let Some(parent) = cred_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config dir: {}", parent.display()))?;
    }

    let creds = json!({
        "server": server_url,
        "access_token": login.access_token,
        "expires_at": login.expires_at,
        "email": email,
    });

    // Write with restrictive permissions (owner-only read/write)
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&cred_path)
            .with_context(|| {
                format!("Failed to write credentials to {}", cred_path.display())
            })?;
        serde_json::to_writer_pretty(file, &creds)?;
    }
    #[cfg(not(unix))]
    {
        let file = std::fs::File::create(&cred_path)
            .with_context(|| format!("Failed to write credentials to {}", cred_path.display()))?;
        serde_json::to_writer_pretty(file, &creds)?;
    }

    Ok(cred_path)
}

async fn authenticate_register(args: RegisterAuthArgs) -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct RegisterResponse {
        user_id: String,
        email: String,
        message: String,
        #[serde(default)]
        instance_url: Option<String>,
        #[serde(default)]
        instance_namespace: Option<String>,
        #[serde(default)]
        provisioning_status: Option<String>,
    }

    let server_url = args.server.trim_end_matches('/').to_string();

    let email = match args.email {
        Some(e) => e,
        None => {
            print!("Email: ");
            io::stdout().flush()?;
            let mut email = String::new();
            io::stdin().read_line(&mut email)?;
            email.trim().to_string()
        }
    };

    if email.is_empty() {
        anyhow::bail!("Email is required");
    }

    let password = rpassword_prompt("Password (min 8 chars): ")?;
    if password.trim().len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }
    let confirm = rpassword_prompt("Confirm password: ")?;
    if password != confirm {
        anyhow::bail!("Passwords do not match");
    }

    println!("Registering with {}...", server_url);

    let client = Client::new();
    let user_agent = format!("codetether-agent/{}", env!("CARGO_PKG_VERSION"));

    let resp = client
        .post(format!("{}/v1/users/register", server_url))
        .header("User-Agent", &user_agent)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password,
            "first_name": args.first_name,
            "last_name": args.last_name,
            "referral_source": args.referral_source,
        }))
        .send()
        .await
        .context("Failed to connect to CodeTether server")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        let detail = body
            .get("detail")
            .and_then(|v| v.as_str())
            .unwrap_or("Registration failed");
        anyhow::bail!("Registration failed ({}): {}", status, detail);
    }

    let reg: RegisterResponse = resp
        .json()
        .await
        .context("Failed to parse registration response")?;

    println!("Account created for {} (user_id={})", reg.email, reg.user_id);
    println!("{}", reg.message);
    if let Some(status) = reg.provisioning_status.as_deref() {
        println!("Provisioning status: {}", status);
    }
    if let Some(url) = reg.instance_url.as_deref() {
        println!("Instance URL: {}", url);
    }
    if let Some(ns) = reg.instance_namespace.as_deref() {
        println!("Instance namespace: {}", ns);
    }

    // Auto-login and save credentials for the worker.
    println!("Logging in...");
    let login = login_with_password(&client, &server_url, &reg.email, &password).await?;
    let cred_path = write_saved_credentials(&server_url, &reg.email, &login)?;

    let user_email = login
        .user
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or(&reg.email);

    println!("Logged in as {} (expires {})", user_email, login.expires_at);
    println!("Credentials saved to {}", cred_path.display());
    println!("\nThe CLI will automatically use these credentials for `codetether worker`.");

    Ok(())
}

async fn authenticate_login(args: LoginAuthArgs) -> Result<()> {
    let server_url = args.server.trim_end_matches('/').to_string();

    // Prompt for email if not provided
    let email = match args.email {
        Some(e) => e,
        None => {
            print!("Email: ");
            io::stdout().flush()?;
            let mut email = String::new();
            io::stdin().read_line(&mut email)?;
            email.trim().to_string()
        }
    };

    if email.is_empty() {
        anyhow::bail!("Email is required");
    }

    // Prompt for password (no echo)
    let password = rpassword_prompt("Password: ")?;
    if password.is_empty() {
        anyhow::bail!("Password is required");
    }

    println!("Authenticating with {}...", server_url);

    let client = Client::new();

    let login = login_with_password(&client, &server_url, &email, &password).await?;
    let cred_path = write_saved_credentials(&server_url, &email, &login)?;

    let user_email = login
        .user
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or(&email);

    println!("Logged in as {} (expires {})", user_email, login.expires_at);
    println!("Credentials saved to {}", cred_path.display());
    println!("\nThe CLI will automatically use these credentials for `codetether worker`.");

    Ok(())
}

/// Read password from terminal without echo.
fn rpassword_prompt(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    // Disable echo on Unix
    #[cfg(unix)]
    {
        use std::io::BufRead;
        // Save terminal state
        let fd = 0; // stdin
        let orig = unsafe {
            let mut termios = std::mem::zeroed::<libc::termios>();
            libc::tcgetattr(fd, &mut termios);
            termios
        };

        // Disable echo
        unsafe {
            let mut termios = orig;
            termios.c_lflag &= !libc::ECHO;
            libc::tcsetattr(fd, libc::TCSANOW, &termios);
        }

        let mut password = String::new();
        let result = io::stdin().lock().read_line(&mut password);

        // Restore terminal state
        unsafe {
            libc::tcsetattr(fd, libc::TCSANOW, &orig);
        }
        println!(); // newline after password entry

        result?;
        Ok(password.trim().to_string())
    }

    #[cfg(not(unix))]
    {
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;
        Ok(password.trim().to_string())
    }
}

/// Get the path to the credential storage file.
fn credential_file_path() -> Result<std::path::PathBuf> {
    use directories::ProjectDirs;
    let dirs = ProjectDirs::from("ai", "codetether", "codetether-agent")
        .ok_or_else(|| anyhow::anyhow!("Cannot determine config directory"))?;
    Ok(dirs.config_dir().join("credentials.json"))
}

/// Stored credentials from `codetether auth login`.
#[derive(Debug, Deserialize)]
pub struct SavedCredentials {
    pub server: String,
    pub access_token: String,
    pub expires_at: String,
    #[serde(default)]
    pub email: String,
}

/// Load saved credentials from disk, returning `None` if the file doesn't exist,
/// is malformed, or the token has expired.
pub fn load_saved_credentials() -> Option<SavedCredentials> {
    let path = credential_file_path().ok()?;
    let data = std::fs::read_to_string(&path).ok()?;
    let creds: SavedCredentials = serde_json::from_str(&data).ok()?;

    // Check expiry if parseable
    if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(&creds.expires_at) {
        if expires < chrono::Utc::now() {
            tracing::warn!("Saved credentials have expired — run `codetether auth login` to refresh");
            return None;
        }
    }

    Some(creds)
}

async fn authenticate_copilot(args: CopilotAuthArgs) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN before running `codetether auth copilot`."
        );
    }

    let (provider_id, domain, enterprise_domain) = match args.enterprise_url {
        Some(raw) => {
            let domain = normalize_enterprise_domain(&raw);
            if domain.is_empty() {
                anyhow::bail!("--enterprise-url cannot be empty");
            }
            ("github-copilot-enterprise", domain.clone(), Some(domain))
        }
        None => ("github-copilot", DEFAULT_GITHUB_DOMAIN.to_string(), None),
    };

    let client = Client::new();
    let client_id = resolve_client_id(args.client_id)?;
    let user_agent = format!("codetether-agent/{}", env!("CARGO_PKG_VERSION"));
    let device = request_device_code(&client, &domain, &user_agent, &client_id).await?;

    println!("GitHub Copilot device authentication");
    println!(
        "Open this URL: {}",
        device
            .verification_uri_complete
            .as_deref()
            .unwrap_or(&device.verification_uri)
    );
    println!("Enter code: {}", device.user_code);
    println!("Waiting for authorization...");

    let token = poll_for_access_token(&client, &domain, &user_agent, &client_id, &device).await?;

    let mut extra = HashMap::new();
    if let Some(enterprise_url) = enterprise_domain {
        extra.insert(
            "enterpriseUrl".to_string(),
            serde_json::Value::String(enterprise_url),
        );
    }

    let provider_secrets = ProviderSecrets {
        api_key: Some(token),
        base_url: None,
        organization: None,
        headers: None,
        extra,
    };

    secrets::set_provider_secrets(provider_id, &provider_secrets)
        .await
        .with_context(|| format!("Failed to store {} auth token in Vault", provider_id))?;

    println!("Saved {} credentials to HashiCorp Vault.", provider_id);
    Ok(())
}

async fn request_device_code(
    client: &Client,
    domain: &str,
    user_agent: &str,
    client_id: &str,
) -> Result<DeviceCodeResponse> {
    let url = format!("https://{domain}/login/device/code");
    let response = client
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", user_agent)
        .json(&json!({
            "client_id": client_id,
            "scope": "read:user",
        }))
        .send()
        .await
        .with_context(|| format!("Failed to reach device authorization endpoint: {url}"))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Failed to initiate device authorization ({}): {}",
            status,
            truncate_body(&body)
        );
    }

    let mut device: DeviceCodeResponse = response
        .json()
        .await
        .context("Failed to parse device authorization response")?;
    if device.interval.unwrap_or(0) == 0 {
        device.interval = Some(5);
    }
    Ok(device)
}

async fn poll_for_access_token(
    client: &Client,
    domain: &str,
    user_agent: &str,
    client_id: &str,
    device: &DeviceCodeResponse,
) -> Result<String> {
    let url = format!("https://{domain}/login/oauth/access_token");
    let mut interval_secs = device.interval.unwrap_or(5).max(1);

    loop {
        let response = client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", user_agent)
            .json(&json!({
                "client_id": client_id,
                "device_code": device.device_code,
                "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
            }))
            .send()
            .await
            .with_context(|| format!("Failed to poll token endpoint: {url}"))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Failed to exchange device code for access token ({}): {}",
                status,
                truncate_body(&body)
            );
        }

        let payload: AccessTokenResponse = response
            .json()
            .await
            .context("Failed to parse OAuth token response")?;

        if let Some(token) = payload.access_token {
            if !token.trim().is_empty() {
                return Ok(token);
            }
        }

        match payload.error.as_deref() {
            Some("authorization_pending") => sleep_with_margin(interval_secs).await,
            Some("slow_down") => {
                interval_secs = payload
                    .interval
                    .filter(|value| *value > 0)
                    .unwrap_or(interval_secs + 5);
                sleep_with_margin(interval_secs).await;
            }
            Some(error) => {
                let description = payload
                    .error_description
                    .unwrap_or_else(|| "No error description provided".to_string());
                anyhow::bail!("Copilot OAuth failed: {} ({})", error, description);
            }
            None => sleep_with_margin(interval_secs).await,
        }
    }
}

fn resolve_client_id(client_id: Option<String>) -> Result<String> {
    let id = client_id
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "GitHub OAuth client ID is required. Pass `--client-id <id>` or set `CODETETHER_COPILOT_OAUTH_CLIENT_ID`."
            )
        })?;

    Ok(id)
}

async fn sleep_with_margin(interval_secs: u64) {
    sleep(Duration::from_millis(
        interval_secs.saturating_mul(1000) + OAUTH_POLLING_SAFETY_MARGIN_MS,
    ))
    .await;
}

fn truncate_body(body: &str) -> String {
    const MAX_LEN: usize = 300;
    if body.len() <= MAX_LEN {
        body.to_string()
    } else {
        format!("{}...", &body[..MAX_LEN])
    }
}
