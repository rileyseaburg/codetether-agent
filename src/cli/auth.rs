//! Provider authentication commands.

use super::{
    AuthArgs, AuthCommand, CodexAuthArgs, CopilotAuthArgs, LoginAuthArgs, RegisterAuthArgs,
};
use crate::provider::copilot::normalize_enterprise_domain;
use crate::provider::openai_codex::OpenAiCodexProvider;
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::{Duration, Instant, sleep};

const DEFAULT_GITHUB_DOMAIN: &str = "github.com";
const OAUTH_POLLING_SAFETY_MARGIN_MS: u64 = 3000;
const CODEX_CALLBACK_ADDR: &str = "127.0.0.1:1455";
const CODEX_CALLBACK_TIMEOUT_SECS: u64 = 300;
const CODEX_CALLBACK_TIMEOUT_SSH_SECS: u64 = 15;

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
        AuthCommand::Codex(codex_args) => authenticate_codex(codex_args).await,
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
            .with_context(|| format!("Failed to write credentials to {}", cred_path.display()))?;
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

    println!(
        "Account created for {} (user_id={})",
        reg.email, reg.user_id
    );
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
            tracing::warn!(
                "Saved credentials have expired — run `codetether auth login` to refresh"
            );
            return None;
        }
    }

    Some(creds)
}

async fn authenticate_codex(_args: CodexAuthArgs) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN before running `codetether auth codex`."
        );
    }

    let (authorization_url, code_verifier, expected_state) =
        OpenAiCodexProvider::get_authorization_url();

    println!("OpenAI Codex OAuth authentication");
    println!(
        "Sign in with your ChatGPT subscription account (Plus/Pro/Team/Enterprise) to use Codex models without API credits."
    );

    let is_ssh_session =
        std::env::var_os("SSH_CONNECTION").is_some() || std::env::var_os("SSH_TTY").is_some();
    if is_ssh_session {
        println!("Detected SSH session.");
        println!(
            "If your browser runs on your local machine, port-forward callback traffic first:"
        );
        println!("  ssh -L 1455:127.0.0.1:1455 <remote-host>");
        println!("Without forwarding, manual callback paste is still supported.");
    }

    println!("Open this URL: {}", authorization_url);
    println!(
        "After approving access, copy the browser callback URL and paste it below (it starts with http://localhost:1455/auth/callback)."
    );
    let callback_timeout = if is_ssh_session {
        Duration::from_secs(CODEX_CALLBACK_TIMEOUT_SSH_SECS)
    } else {
        Duration::from_secs(CODEX_CALLBACK_TIMEOUT_SECS)
    };
    let auto_callback = capture_oauth_callback_auto(callback_timeout).await?;
    let (authorization_code, returned_state) = if let Some(callback) = auto_callback {
        println!("Captured callback automatically.");
        callback
    } else {
        let callback_input = prompt_line("Callback URL: ")?;
        extract_oauth_code_and_state(&callback_input)?
    };

    if returned_state != expected_state {
        anyhow::bail!(
            "OAuth state mismatch. Retry `codetether auth codex` and paste the callback URL from the same login attempt."
        );
    }

    let credentials = OpenAiCodexProvider::exchange_code(&authorization_code, &code_verifier)
        .await
        .context("Failed to exchange ChatGPT OAuth code for Codex tokens")?;

    let mut extra = HashMap::new();
    extra.insert(
        "access_token".to_string(),
        serde_json::Value::String(credentials.access_token),
    );
    extra.insert(
        "refresh_token".to_string(),
        serde_json::Value::String(credentials.refresh_token),
    );
    extra.insert(
        "expires_at".to_string(),
        serde_json::Value::Number(credentials.expires_at.into()),
    );

    let provider_secrets = ProviderSecrets {
        api_key: None,
        base_url: None,
        organization: None,
        headers: None,
        extra,
    };

    secrets::set_provider_secrets("openai-codex", &provider_secrets)
        .await
        .context("Failed to store openai-codex OAuth credentials in Vault")?;

    let expires_display = chrono::DateTime::from_timestamp(credentials.expires_at as i64, 0)
        .map(|ts| ts.to_rfc3339())
        .unwrap_or_else(|| credentials.expires_at.to_string());

    println!("Saved openai-codex credentials to HashiCorp Vault.");
    println!("Access token expires at {}", expires_display);
    println!("You can now select models like `openai-codex/gpt-5-codex`.");
    Ok(())
}

async fn capture_oauth_callback_auto(timeout: Duration) -> Result<Option<(String, String)>> {
    let listener = match TcpListener::bind(CODEX_CALLBACK_ADDR).await {
        Ok(listener) => listener,
        Err(error) => {
            tracing::warn!(
                address = CODEX_CALLBACK_ADDR,
                error = %error,
                "Failed to bind OAuth callback listener; falling back to manual paste"
            );
            return Ok(None);
        }
    };

    println!(
        "Waiting up to {}s for automatic callback capture on http://{}/auth/callback ...",
        timeout.as_secs(),
        CODEX_CALLBACK_ADDR
    );

    match wait_for_oauth_callback(listener, timeout).await {
        Ok(callback) => Ok(Some(callback)),
        Err(error) => {
            tracing::warn!(
                error = %error,
                "Automatic OAuth callback capture did not complete; falling back to manual paste"
            );
            Ok(None)
        }
    }
}

async fn wait_for_oauth_callback(
    listener: TcpListener,
    timeout: Duration,
) -> Result<(String, String)> {
    let deadline = Instant::now() + timeout;

    loop {
        let now = Instant::now();
        if now >= deadline {
            anyhow::bail!("Timed out waiting for OAuth callback");
        }
        let remaining = deadline - now;

        let (mut stream, peer_addr) = tokio::time::timeout(remaining, listener.accept())
            .await
            .context("Timed out waiting for callback connection")?
            .context("Failed to accept callback connection")?;

        let request = read_http_request(&mut stream).await?;
        match parse_oauth_callback_request(&request) {
            Ok((code, state)) => {
                write_http_response(
                    &mut stream,
                    200,
                    "OK",
                    "<html><body><h1>CodeTether login complete</h1><p>You can close this tab.</p></body></html>",
                )
                .await?;
                return Ok((code, state));
            }
            Err(error) => {
                tracing::warn!(
                    peer = %peer_addr,
                    error = %error,
                    "Ignoring non-callback HTTP request while waiting for OAuth callback"
                );
                write_http_response(
                    &mut stream,
                    400,
                    "Bad Request",
                    "<html><body><h1>Invalid callback request</h1><p>Retry authorization from CodeTether.</p></body></html>",
                )
                .await?;
            }
        }
    }
}

async fn read_http_request(stream: &mut tokio::net::TcpStream) -> Result<String> {
    let mut buffer = [0u8; 8192];
    let read = stream
        .read(&mut buffer)
        .await
        .context("Failed to read callback request")?;
    if read == 0 {
        anyhow::bail!("Callback request stream closed before data was received");
    }
    Ok(String::from_utf8_lossy(&buffer[..read]).to_string())
}

fn parse_oauth_callback_request(request: &str) -> Result<(String, String)> {
    let first_line = request
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing HTTP request line"))?;
    let mut parts = first_line.split_whitespace();

    let method = parts.next().unwrap_or_default();
    if method != "GET" {
        anyhow::bail!("Unsupported callback method: {}", method);
    }

    let target = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing callback target"))?;
    let query = target
        .split_once('?')
        .map(|(_, query)| query)
        .ok_or_else(|| anyhow::anyhow!("Callback target missing query string"))?;

    extract_oauth_code_and_state(query)
}

async fn write_http_response(
    stream: &mut tokio::net::TcpStream,
    status_code: u16,
    status_text: &str,
    body: &str,
) -> Result<()> {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    );
    stream
        .write_all(response.as_bytes())
        .await
        .context("Failed to write callback response")?;
    Ok(())
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

fn prompt_line(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        anyhow::bail!("Input is required");
    }
    Ok(trimmed)
}

fn extract_oauth_code_and_state(callback_input: &str) -> Result<(String, String)> {
    let input = callback_input.trim();
    if input.is_empty() {
        anyhow::bail!("Callback URL is required");
    }

    let query = if input.contains("://") {
        let url =
            reqwest::Url::parse(input).with_context(|| format!("Invalid callback URL: {input}"))?;
        url.query()
            .map(str::to_string)
            .ok_or_else(|| anyhow::anyhow!("Callback URL is missing query parameters"))?
    } else if let Some((_, params)) = input.split_once('?') {
        params.to_string()
    } else {
        input.to_string()
    };

    let params = parse_query_pairs(&query);
    if let Some(error) = params.get("error") {
        let error_description = params
            .get("error_description")
            .map(String::as_str)
            .unwrap_or("No error description provided");
        anyhow::bail!(
            "OAuth authorization failed: {} ({})",
            error,
            error_description
        );
    }

    let code = params
        .get("code")
        .cloned()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Callback URL does not include an OAuth code"))?;
    let state = params
        .get("state")
        .cloned()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Callback URL does not include OAuth state"))?;

    Ok((code, state))
}

fn parse_query_pairs(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    for pair in query.split('&') {
        if pair.trim().is_empty() {
            continue;
        }

        let (raw_key, raw_value) = match pair.split_once('=') {
            Some((key, value)) => (key, value),
            None => (pair, ""),
        };
        let key = decode_query_component(raw_key);
        let value = decode_query_component(raw_value);
        params.insert(key, value);
    }

    params
}

fn decode_query_component(component: &str) -> String {
    match urlencoding::decode(component) {
        Ok(value) => value.into_owned(),
        Err(_) => component.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_oauth_code_and_state, parse_oauth_callback_request};

    #[test]
    fn extracts_code_and_state_from_full_callback_url() {
        let input = "http://localhost:1455/auth/callback?code=abc123&state=xyz789";
        let (code, state) = extract_oauth_code_and_state(input).expect("expected callback parse");
        assert_eq!(code, "abc123");
        assert_eq!(state, "xyz789");
    }

    #[test]
    fn extracts_code_and_state_from_raw_query_string() {
        let input = "code=abc123&state=xyz789";
        let (code, state) = extract_oauth_code_and_state(input).expect("expected callback parse");
        assert_eq!(code, "abc123");
        assert_eq!(state, "xyz789");
    }

    #[test]
    fn returns_error_when_state_is_missing() {
        let input = "http://localhost:1455/auth/callback?code=abc123";
        let err = extract_oauth_code_and_state(input).expect_err("expected missing state");
        assert!(err.to_string().contains("OAuth state"));
    }

    #[test]
    fn parses_oauth_callback_http_request() {
        let request =
            "GET /auth/callback?code=abc123&state=xyz789 HTTP/1.1\r\nHost: localhost:1455\r\n\r\n";
        let (code, state) =
            parse_oauth_callback_request(request).expect("expected valid callback request");
        assert_eq!(code, "abc123");
        assert_eq!(state, "xyz789");
    }

    #[test]
    fn rejects_non_get_callback_request() {
        let request =
            "POST /auth/callback?code=abc123&state=xyz789 HTTP/1.1\r\nHost: localhost:1455\r\n\r\n";
        let err =
            parse_oauth_callback_request(request).expect_err("expected non-GET callback rejection");
        assert!(err.to_string().contains("Unsupported callback method"));
    }
}
