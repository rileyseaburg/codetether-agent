//! Provider authentication commands.

use super::{
    AuthArgs, AuthCommand, CodexAuthArgs, CookieAuthArgs, CopilotAuthArgs, LoginAuthArgs,
    RegisterAuthArgs,
};
use crate::provider::copilot::normalize_enterprise_domain;
use crate::provider::openai_codex::{OAuthCredentials, OpenAiCodexProvider};
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::time::Duration;

const DEFAULT_GITHUB_DOMAIN: &str = "github.com";
const CODEX_CALLBACK_TIMEOUT_SECS: u64 = 300;
const CODEX_CALLBACK_TIMEOUT_SSH_SECS: u64 = 15;

pub async fn execute(args: AuthArgs) -> Result<()> {
    match args.command {
        AuthCommand::Copilot(copilot_args) => authenticate_copilot(copilot_args).await,
        AuthCommand::Bedrock(bedrock_args) => crate::cli::auth_bedrock::execute(bedrock_args).await,
        AuthCommand::Codex(codex_args) => authenticate_codex(codex_args).await,
        AuthCommand::Cookies(cookie_args) => authenticate_cookie_import(cookie_args).await,
        AuthCommand::Register(register_args) => authenticate_register(register_args).await,
        AuthCommand::Login(login_args) => authenticate_login(login_args).await,
    }
}

async fn authenticate_codex(args: CodexAuthArgs) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN before running `codetether auth codex`."
        );
    }
    secrets::verify_reachable().await.context(
        "HashiCorp Vault is configured but unreachable. Start Vault, correct VAULT_ADDR/VAULT_TOKEN, or unset stale Vault env before running `codetether auth codex`.",
    )?;

    if args.device_code {
        let credentials = authenticate_codex_device_code().await?;
        return store_codex_credentials(credentials).await;
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
        if is_ssh_session {
            println!(
                "Press Enter to switch to device-code auth, or paste callback URL from your browser."
            );
        }
        let callback_input = if is_ssh_session {
            prompt_optional_line("Callback URL: ")?
        } else {
            prompt_line("Callback URL: ")?
        };

        if callback_input.trim().is_empty() {
            let credentials = authenticate_codex_device_code().await?;
            return store_codex_credentials(credentials).await;
        }

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

    store_codex_credentials(credentials).await
}

async fn store_codex_credentials(credentials: OAuthCredentials) -> Result<()> {
    let chatgpt_account_id = credentials
        .chatgpt_account_id
        .clone()
        .or_else(|| {
            credentials
                .id_token
                .as_deref()
                .and_then(OpenAiCodexProvider::extract_chatgpt_account_id)
        })
        .or_else(|| OpenAiCodexProvider::extract_chatgpt_account_id(&credentials.access_token));

    let mut expected_token_exchange_fallback = false;
    let api_key = if let Some(id_token) = credentials.id_token.as_deref() {
        match OpenAiCodexProvider::exchange_id_token_for_api_key(id_token).await {
            Ok(key) => Some(key),
            Err(error) => {
                if is_expected_codex_id_token_exchange_fallback(&error) {
                    expected_token_exchange_fallback = true;
                    tracing::info!(
                        error = %error,
                        "Expected id_token exchange fallback; using OAuth access token for Codex backend"
                    );
                } else {
                    tracing::warn!(
                        error = %error,
                        "Failed to exchange id_token for OpenAI API key; falling back to OAuth access token"
                    );
                }
                None
            }
        }
    } else {
        tracing::warn!(
            "OAuth token exchange did not return an id_token; cannot derive OpenAI API key"
        );
        None
    };

    let mut extra = HashMap::new();
    extra.insert(
        "access_token".to_string(),
        serde_json::Value::String(credentials.access_token.clone()),
    );
    extra.insert(
        "refresh_token".to_string(),
        serde_json::Value::String(credentials.refresh_token.clone()),
    );
    extra.insert(
        "expires_at".to_string(),
        serde_json::Value::Number(credentials.expires_at.into()),
    );
    if let Some(id_token) = credentials.id_token.as_ref() {
        extra.insert(
            "id_token".to_string(),
            serde_json::Value::String(id_token.clone()),
        );
    }
    if let Some(account_id) = chatgpt_account_id.as_ref() {
        extra.insert(
            "chatgpt_account_id".to_string(),
            serde_json::Value::String(account_id.clone()),
        );
    }

    let provider_secrets = ProviderSecrets {
        api_key: api_key.clone(),
        base_url: None,
        organization: chatgpt_account_id.clone(),
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
    if api_key.is_some() {
        println!("Stored exchanged OpenAI API key for Codex model requests.");
    } else {
        println!(
            "Could not exchange an OpenAI API key; Codex requests will use ChatGPT OAuth backend tokens."
        );
        if expected_token_exchange_fallback {
            println!(
                "Note: this fallback is expected when your id_token does not include organization context."
            );
        }
    }
    if let Some(account_id) = chatgpt_account_id {
        println!("Using ChatGPT workspace/account ID: {}", account_id);
    }
    println!("Access token expires at {}", expires_display);
    println!("You can now select models like `openai-codex/gpt-5.5`.");
    Ok(())
}

fn is_expected_codex_id_token_exchange_fallback(error: &anyhow::Error) -> bool {
    let msg = error.to_string().to_ascii_lowercase();
    msg.contains("missing organization_id")
        || (msg.contains("invalid_subject_token") && msg.contains("organization"))
}

async fn authenticate_codex_device_code() -> Result<OAuthCredentials> {
    let client = Client::new();
    let issuer = OpenAiCodexProvider::oauth_issuer().trim_end_matches('/');
    let user_agent = format!("codetether-agent/{}", env!("CARGO_PKG_VERSION"));
    let device_code = request_codex_device_code(&client, issuer, &user_agent).await?;

    println!("OpenAI Codex device authentication");
    println!("Open this URL: {issuer}/codex/device");
    println!("Enter code: {}", device_code.user_code);
    println!("Waiting for authorization...");

    let code = poll_for_codex_authorization_code(&client, issuer, &user_agent, &device_code)
        .await
        .context("Timed out waiting for device authorization")?;

    let redirect_uri = format!("{issuer}/deviceauth/callback");
    OpenAiCodexProvider::exchange_code_with_redirect_uri(
        &code.authorization_code,
        &code.code_verifier,
        &redirect_uri,
    )
    .await
    .context("Failed to exchange device authorization code for Codex tokens")
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

fn prompt_optional_line(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub(crate) use codetether_user_auth::{LoginResponsePayload, write_saved_credentials};
pub use codetether_user_auth::{SavedCredentials, load_saved_credentials};

async fn login_with_password(
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

use codetether_cookie_auth_cli::execute as authenticate_cookie_import;

use codetether_oauth_callback::{capture_oauth_callback_auto, extract_oauth_code_and_state};

use codetether_device_auth::{
    poll_for_access_token, poll_for_codex_authorization_code, request_device_code,
};

async fn request_codex_device_code(
    client: &Client,
    issuer: &str,
    user_agent: &str,
) -> Result<codetether_device_auth::CodexDeviceCodeResponse> {
    codetether_device_auth::request_codex_device_code(
        client,
        issuer,
        user_agent,
        OpenAiCodexProvider::oauth_client_id(),
    )
    .await
}

pub(crate) use codetether_user_auth_cli::rpassword_prompt;

async fn authenticate_register(args: RegisterAuthArgs) -> Result<()> {
    codetether_user_auth_cli::register(args, env!("CARGO_PKG_VERSION")).await
}

async fn authenticate_login(args: LoginAuthArgs) -> Result<()> {
    codetether_user_auth_cli::login(args, env!("CARGO_PKG_VERSION")).await
}
