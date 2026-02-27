//! OpenAI Codex provider using ChatGPT Plus/Pro subscription via OAuth
//!
//! This provider uses the OAuth PKCE flow that the official OpenAI Codex CLI uses,
//! allowing users to authenticate with their ChatGPT subscription instead of API credits.
//!
//! Reference: https://github.com/numman-ali/opencode-openai-codex-auth

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use futures::StreamExt;
use futures::stream::BoxStream;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

const OPENAI_API_URL: &str = "https://api.openai.com/v1";
const CHATGPT_CODEX_API_URL: &str = "https://chatgpt.com/backend-api/codex";
const AUTH_ISSUER: &str = "https://auth.openai.com";
const AUTHORIZE_URL: &str = "https://auth.openai.com/oauth/authorize";
const TOKEN_URL: &str = "https://auth.openai.com/oauth/token";
const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
const REDIRECT_URI: &str = "http://localhost:1455/auth/callback";
const SCOPE: &str = "openid profile email offline_access";
const THINKING_LEVEL_ENV: &str = "CODETETHER_OPENAI_CODEX_THINKING_LEVEL";
const REASONING_EFFORT_ENV: &str = "CODETETHER_OPENAI_CODEX_REASONING_EFFORT";
const DEFAULT_RESPONSES_INSTRUCTIONS: &str = "You are a helpful assistant.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThinkingLevel {
    Low,
    Medium,
    High,
}

impl ThinkingLevel {
    fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }
}

/// Cached OAuth tokens with expiration tracking
#[allow(dead_code)]
struct CachedTokens {
    access_token: String,
    #[allow(dead_code)]
    refresh_token: String,
    expires_at: std::time::Instant,
}

/// Stored OAuth credentials (persisted to Vault)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCredentials {
    #[serde(default)]
    pub id_token: Option<String>,
    #[serde(default)]
    pub chatgpt_account_id: Option<String>,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64, // Unix timestamp in seconds
}

/// PKCE code verifier and challenge pair
struct PkcePair {
    verifier: String,
    challenge: String,
}

pub struct OpenAiCodexProvider {
    client: Client,
    cached_tokens: Arc<RwLock<Option<CachedTokens>>>,
    static_api_key: Option<String>,
    chatgpt_account_id: Option<String>,
    /// Stored credentials from Vault (for refresh on startup)
    stored_credentials: Option<OAuthCredentials>,
}

impl std::fmt::Debug for OpenAiCodexProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiCodexProvider")
            .field("has_api_key", &self.static_api_key.is_some())
            .field("has_chatgpt_account_id", &self.chatgpt_account_id.is_some())
            .field("has_credentials", &self.stored_credentials.is_some())
            .finish()
    }
}

impl OpenAiCodexProvider {
    pub fn from_api_key(api_key: String) -> Self {
        Self {
            client: Client::new(),
            cached_tokens: Arc::new(RwLock::new(None)),
            static_api_key: Some(api_key),
            chatgpt_account_id: None,
            stored_credentials: None,
        }
    }

    /// Create from stored OAuth credentials (from Vault)
    pub fn from_credentials(credentials: OAuthCredentials) -> Self {
        let chatgpt_account_id = credentials
            .chatgpt_account_id
            .clone()
            .or_else(|| {
                credentials
                    .id_token
                    .as_deref()
                    .and_then(Self::extract_chatgpt_account_id_from_jwt)
            })
            .or_else(|| Self::extract_chatgpt_account_id_from_jwt(&credentials.access_token));

        Self {
            client: Client::new(),
            cached_tokens: Arc::new(RwLock::new(None)),
            static_api_key: None,
            chatgpt_account_id,
            stored_credentials: Some(credentials),
        }
    }

    /// Create a new unauthenticated instance (requires OAuth flow)
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cached_tokens: Arc::new(RwLock::new(None)),
            static_api_key: None,
            chatgpt_account_id: None,
            stored_credentials: None,
        }
    }

    /// Generate PKCE code verifier and challenge
    fn generate_pkce() -> PkcePair {
        let random_bytes: [u8; 32] = {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);

            let mut bytes = [0u8; 32];
            let ts_bytes = timestamp.to_le_bytes();
            let tid = std::thread::current().id();
            let tid_repr = format!("{:?}", tid);
            let tid_hash = Sha256::digest(tid_repr.as_bytes());

            bytes[0..8].copy_from_slice(&ts_bytes[0..8]);
            bytes[8..24].copy_from_slice(&tid_hash[0..16]);
            bytes[24..].copy_from_slice(&Sha256::digest(ts_bytes)[0..8]);
            bytes
        };
        let verifier = URL_SAFE_NO_PAD.encode(random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge_bytes = hasher.finalize();
        let challenge = URL_SAFE_NO_PAD.encode(challenge_bytes);

        PkcePair {
            verifier,
            challenge,
        }
    }

    /// Generate random state value
    fn generate_state() -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let random: [u8; 8] = {
            let ptr = Box::into_raw(Box::new(timestamp)) as usize;
            let bytes = ptr.to_le_bytes();
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&bytes);
            arr
        };
        format!("{:016x}{:016x}", timestamp, u64::from_le_bytes(random))
    }

    /// Get the OAuth authorization URL for the user to visit
    #[allow(dead_code)]
    pub fn get_authorization_url() -> (String, String, String) {
        let pkce = Self::generate_pkce();
        let state = Self::generate_state();

        let url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&code_challenge={}&code_challenge_method=S256&state={}&id_token_add_organizations=true&codex_cli_simplified_flow=true&originator=codex_cli_rs",
            AUTHORIZE_URL,
            CLIENT_ID,
            urlencoding::encode(REDIRECT_URI),
            urlencoding::encode(SCOPE),
            pkce.challenge,
            state
        );

        (url, pkce.verifier, state)
    }

    pub fn oauth_issuer() -> &'static str {
        AUTH_ISSUER
    }

    pub fn oauth_client_id() -> &'static str {
        CLIENT_ID
    }

    /// Exchange authorization code for tokens
    #[allow(dead_code)]
    pub async fn exchange_code(code: &str, verifier: &str) -> Result<OAuthCredentials> {
        Self::exchange_code_with_redirect_uri(code, verifier, REDIRECT_URI).await
    }

    /// Exchange authorization code for tokens using a custom redirect URI.
    pub async fn exchange_code_with_redirect_uri(
        code: &str,
        verifier: &str,
        redirect_uri: &str,
    ) -> Result<OAuthCredentials> {
        let client = Client::new();
        let form_body = format!(
            "grant_type={}&client_id={}&code={}&code_verifier={}&redirect_uri={}",
            urlencoding::encode("authorization_code"),
            CLIENT_ID,
            urlencoding::encode(code),
            urlencoding::encode(verifier),
            urlencoding::encode(redirect_uri),
        );

        let response = client
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .context("Failed to exchange authorization code")?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("OAuth token exchange failed: {}", body);
        }

        #[derive(Deserialize)]
        struct TokenResponse {
            #[serde(default)]
            id_token: Option<String>,
            access_token: String,
            refresh_token: String,
            expires_in: u64,
        }

        let tokens: TokenResponse = response
            .json()
            .await
            .context("Failed to parse token response")?;

        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("System time error")?
            .as_secs()
            + tokens.expires_in;

        let chatgpt_account_id = tokens
            .id_token
            .as_deref()
            .and_then(Self::extract_chatgpt_account_id_from_jwt);

        Ok(OAuthCredentials {
            id_token: tokens.id_token,
            chatgpt_account_id,
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_at,
        })
    }

    pub async fn exchange_id_token_for_api_key(id_token: &str) -> Result<String> {
        #[derive(Deserialize)]
        struct ExchangeResponse {
            access_token: String,
        }

        let client = Client::new();
        let form_body = format!(
            "grant_type={}&client_id={}&requested_token={}&subject_token={}&subject_token_type={}",
            urlencoding::encode("urn:ietf:params:oauth:grant-type:token-exchange"),
            urlencoding::encode(CLIENT_ID),
            urlencoding::encode("openai-api-key"),
            urlencoding::encode(id_token),
            urlencoding::encode("urn:ietf:params:oauth:token-type:id_token"),
        );

        let response = client
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .context("Failed to exchange id_token for OpenAI API key")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("API key token exchange failed ({}): {}", status, body);
        }

        let payload: ExchangeResponse = response
            .json()
            .await
            .context("Failed to parse API key token exchange response")?;
        if payload.access_token.trim().is_empty() {
            anyhow::bail!("API key token exchange returned an empty access token");
        }
        Ok(payload.access_token)
    }

    /// Refresh access token using refresh token
    async fn refresh_access_token(&self, refresh_token: &str) -> Result<OAuthCredentials> {
        let form_body = format!(
            "grant_type={}&refresh_token={}&client_id={}",
            urlencoding::encode("refresh_token"),
            urlencoding::encode(refresh_token),
            CLIENT_ID,
        );

        let response = self
            .client
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .context("Failed to refresh access token")?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Token refresh failed: {}", body);
        }

        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: String,
            refresh_token: String,
            expires_in: u64,
        }

        let tokens: TokenResponse = response
            .json()
            .await
            .context("Failed to parse refresh response")?;

        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("System time error")?
            .as_secs()
            + tokens.expires_in;

        Ok(OAuthCredentials {
            id_token: None,
            chatgpt_account_id: self.chatgpt_account_id.clone(),
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_at,
        })
    }

    /// Get a valid access token, refreshing if necessary
    async fn get_access_token(&self) -> Result<String> {
        if let Some(ref api_key) = self.static_api_key {
            return Ok(api_key.clone());
        }

        {
            let cache = self.cached_tokens.read().await;
            if let Some(ref tokens) = *cache
                && tokens
                    .expires_at
                    .duration_since(std::time::Instant::now())
                    .as_secs()
                    > 300
            {
                return Ok(tokens.access_token.clone());
            }
        }

        let mut cache = self.cached_tokens.write().await;

        let creds = if let Some(ref stored) = self.stored_credentials {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .context("System time error")?
                .as_secs();

            if stored.expires_at > now + 300 {
                stored.clone()
            } else {
                self.refresh_access_token(&stored.refresh_token).await?
            }
        } else {
            anyhow::bail!("No OAuth credentials available. Run OAuth flow first.");
        };

        let expires_in = creds.expires_at
            - std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .context("System time error")?
                .as_secs();

        let cached = CachedTokens {
            access_token: creds.access_token.clone(),
            refresh_token: creds.refresh_token.clone(),
            expires_at: std::time::Instant::now() + std::time::Duration::from_secs(expires_in),
        };

        let token = cached.access_token.clone();
        *cache = Some(cached);
        Ok(token)
    }

    #[allow(dead_code)]
    fn convert_messages(messages: &[Message]) -> Vec<Value> {
        messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    Role::Tool => "tool",
                };

                match msg.role {
                    Role::Tool => {
                        if let Some(ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        }) = msg.content.first()
                        {
                            json!({
                                "role": "tool",
                                "tool_call_id": tool_call_id,
                                "content": content
                            })
                        } else {
                            json!({ "role": role, "content": "" })
                        }
                    }
                    Role::Assistant => {
                        let text: String = msg
                            .content
                            .iter()
                            .filter_map(|p| match p {
                                ContentPart::Text { text } => Some(text.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("");

                        let tool_calls: Vec<Value> = msg
                            .content
                            .iter()
                            .filter_map(|p| match p {
                                ContentPart::ToolCall {
                                    id,
                                    name,
                                    arguments,
                                    ..
                                } => Some(json!({
                                    "id": id,
                                    "type": "function",
                                    "function": {
                                        "name": name,
                                        "arguments": arguments
                                    }
                                })),
                                _ => None,
                            })
                            .collect();

                        if tool_calls.is_empty() {
                            json!({ "role": "assistant", "content": text })
                        } else {
                            json!({
                                "role": "assistant",
                                "content": if text.is_empty() { Value::Null } else { json!(text) },
                                "tool_calls": tool_calls
                            })
                        }
                    }
                    _ => {
                        let text: String = msg
                            .content
                            .iter()
                            .filter_map(|p| match p {
                                ContentPart::Text { text } => Some(text.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        json!({ "role": role, "content": text })
                    }
                }
            })
            .collect()
    }

    #[allow(dead_code)]
    fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters
                    }
                })
            })
            .collect()
    }

    fn convert_responses_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "name": t.name,
                    "description": t.description,
                    "strict": false,
                    "parameters": t.parameters
                })
            })
            .collect()
    }

    fn extract_responses_instructions(messages: &[Message]) -> String {
        let instructions = messages
            .iter()
            .filter(|msg| matches!(msg.role, Role::System))
            .filter_map(|msg| {
                let text = msg
                    .content
                    .iter()
                    .filter_map(|p| match p {
                        ContentPart::Text { text } => Some(text.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                (!text.is_empty()).then_some(text)
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        if instructions.is_empty() {
            DEFAULT_RESPONSES_INSTRUCTIONS.to_string()
        } else {
            instructions
        }
    }

    fn convert_messages_to_responses_input(messages: &[Message]) -> Vec<Value> {
        let mut input = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {}
                Role::User => {
                    let text: String = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    if !text.is_empty() {
                        input.push(json!({
                            "type": "message",
                            "role": "user",
                            "content": [{
                                "type": "input_text",
                                "text": text
                            }]
                        }));
                    }
                }
                Role::Assistant => {
                    let text: String = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("");

                    if !text.is_empty() {
                        input.push(json!({
                            "type": "message",
                            "role": "assistant",
                            "content": [{
                                "type": "output_text",
                                "text": text
                            }]
                        }));
                    }

                    for part in &msg.content {
                        if let ContentPart::ToolCall {
                            id,
                            name,
                            arguments,
                            ..
                        } = part
                        {
                            input.push(json!({
                                "type": "function_call",
                                "call_id": id,
                                "name": name,
                                "arguments": arguments,
                            }));
                        }
                    }
                }
                Role::Tool => {
                    for part in &msg.content {
                        if let ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        } = part
                        {
                            input.push(json!({
                                "type": "function_call_output",
                                "call_id": tool_call_id,
                                "output": content,
                            }));
                        }
                    }
                }
            }
        }

        input
    }

    fn using_chatgpt_backend(&self) -> bool {
        self.static_api_key.is_none()
    }

    fn extract_chatgpt_account_id_from_jwt(jwt: &str) -> Option<String> {
        let payload_b64 = jwt.split('.').nth(1)?;
        let payload = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
        let value: Value = serde_json::from_slice(&payload).ok()?;

        value
            .get("https://api.openai.com/auth")
            .and_then(Value::as_object)
            .and_then(|auth| auth.get("chatgpt_account_id"))
            .and_then(Value::as_str)
            .map(|s| s.to_string())
            .or_else(|| {
                value
                    .get("chatgpt_account_id")
                    .and_then(Value::as_str)
                    .map(|s| s.to_string())
            })
            .or_else(|| {
                value
                    .get("organizations")
                    .and_then(Value::as_array)
                    .and_then(|orgs| orgs.first())
                    .and_then(|org| org.get("id"))
                    .and_then(Value::as_str)
                    .map(|s| s.to_string())
            })
    }

    pub fn extract_chatgpt_account_id(token: &str) -> Option<String> {
        Self::extract_chatgpt_account_id_from_jwt(token)
    }

    fn resolved_chatgpt_account_id(&self, access_token: &str) -> Option<String> {
        self.chatgpt_account_id
            .clone()
            .or_else(|| Self::extract_chatgpt_account_id_from_jwt(access_token))
            .or_else(|| {
                self.stored_credentials.as_ref().and_then(|creds| {
                    creds
                        .id_token
                        .as_deref()
                        .and_then(Self::extract_chatgpt_account_id_from_jwt)
                        .or_else(|| creds.chatgpt_account_id.clone())
                })
            })
    }

    fn parse_responses_usage(usage: Option<&Value>) -> Usage {
        let Some(usage) = usage else {
            return Usage::default();
        };

        let prompt_tokens = usage
            .get("prompt_tokens")
            .or_else(|| usage.get("input_tokens"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;

        let completion_tokens = usage
            .get("completion_tokens")
            .or_else(|| usage.get("output_tokens"))
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;

        let total_tokens = usage
            .get("total_tokens")
            .and_then(Value::as_u64)
            .unwrap_or((prompt_tokens + completion_tokens) as u64)
            as usize;

        Usage {
            prompt_tokens,
            completion_tokens,
            total_tokens,
            cache_read_tokens: None,
            cache_write_tokens: None,
        }
    }

    #[allow(dead_code)]
    fn parse_responses_output_parts(output: &[Value]) -> (Vec<ContentPart>, bool) {
        let mut parts = Vec::new();
        let mut has_tool_calls = false;

        for item in output {
            match item.get("type").and_then(Value::as_str) {
                Some("message") => {
                    if let Some(content) = item.get("content").and_then(Value::as_array) {
                        let text = content
                            .iter()
                            .filter(|segment| {
                                matches!(
                                    segment.get("type").and_then(Value::as_str),
                                    Some("output_text") | Some("text")
                                )
                            })
                            .filter_map(|segment| {
                                segment
                                    .get("text")
                                    .and_then(Value::as_str)
                                    .map(str::to_string)
                            })
                            .collect::<Vec<_>>()
                            .join("");
                        if !text.is_empty() {
                            parts.push(ContentPart::Text { text });
                        }
                    }
                }
                Some("function_call") => {
                    let call_id = item
                        .get("call_id")
                        .or_else(|| item.get("id"))
                        .and_then(Value::as_str);
                    let name = item.get("name").and_then(Value::as_str);
                    let arguments = item.get("arguments").and_then(Value::as_str);
                    if let (Some(call_id), Some(name), Some(arguments)) = (call_id, name, arguments)
                    {
                        has_tool_calls = true;
                        parts.push(ContentPart::ToolCall {
                            id: call_id.to_string(),
                            name: name.to_string(),
                            arguments: arguments.to_string(),
                            thought_signature: None,
                        });
                    }
                }
                _ => {}
            }
        }

        (parts, has_tool_calls)
    }

    fn parse_model_thinking_level(model: &str) -> (String, Option<ThinkingLevel>) {
        let Some((base_model, level_str)) = model.rsplit_once(':') else {
            return (model.to_string(), None);
        };
        let Some(level) = ThinkingLevel::parse(level_str) else {
            return (model.to_string(), None);
        };
        if base_model.trim().is_empty() {
            return (model.to_string(), None);
        }
        (base_model.to_string(), Some(level))
    }

    fn env_thinking_level() -> Option<ThinkingLevel> {
        std::env::var(THINKING_LEVEL_ENV)
            .ok()
            .or_else(|| std::env::var(REASONING_EFFORT_ENV).ok())
            .as_deref()
            .and_then(ThinkingLevel::parse)
    }

    fn model_supports_reasoning_effort(model: &str) -> bool {
        let normalized = model.to_ascii_lowercase();
        normalized.starts_with("gpt-5")
            || normalized.starts_with("o1")
            || normalized.starts_with("o3")
            || normalized.starts_with("o4")
    }

    fn resolve_model_and_reasoning_effort(model: &str) -> (String, Option<ThinkingLevel>) {
        let (base_model, level_from_model) = Self::parse_model_thinking_level(model);
        let level = level_from_model.or_else(Self::env_thinking_level);
        if !Self::model_supports_reasoning_effort(&base_model) {
            return (base_model, None);
        }
        (base_model, level)
    }

    fn format_openai_api_error(status: StatusCode, body: &str, model: &str) -> String {
        if status == StatusCode::UNAUTHORIZED && body.contains("Missing scopes: model.request") {
            return format!(
                "OpenAI Codex OAuth token is missing required scope `model.request` for model `{model}`.\n\
                 Re-run `codetether auth codex` and complete OAuth approval with a ChatGPT subscription account \
                 that has model access in your org/project."
            );
        }
        if status == StatusCode::UNAUTHORIZED
            && (body.contains("chatgpt-account-id")
                || body.contains("ChatGPT-Account-ID")
                || body.contains("workspace"))
        {
            return "OpenAI Codex auth is missing a ChatGPT workspace/account identifier.\n\
                    Re-run `codetether auth codex --device-code` and sign in to the intended workspace."
                .to_string();
        }
        format!("OpenAI API error ({status}): {body}")
    }

    async fn complete_with_chatgpt_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
    ) -> Result<CompletionResponse> {
        let stream = self
            .complete_stream_with_chatgpt_responses(request, access_token)
            .await?;
        Self::collect_stream_completion(stream).await
    }

    async fn complete_with_openai_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
    ) -> Result<CompletionResponse> {
        let stream = self
            .complete_stream_with_openai_responses(request, api_key)
            .await?;
        Self::collect_stream_completion(stream).await
    }

    async fn collect_stream_completion(
        mut stream: BoxStream<'static, StreamChunk>,
    ) -> Result<CompletionResponse> {
        #[derive(Default)]
        struct ToolAccumulator {
            id: String,
            name: String,
            arguments: String,
        }

        let mut text = String::new();
        let mut tools = Vec::<ToolAccumulator>::new();
        let mut tool_index_by_id = HashMap::<String, usize>::new();
        let mut usage = Usage::default();

        while let Some(chunk) = stream.next().await {
            match chunk {
                StreamChunk::Text(delta) => text.push_str(&delta),
                StreamChunk::ToolCallStart { id, name } => {
                    let next_idx = tools.len();
                    let idx = *tool_index_by_id.entry(id.clone()).or_insert(next_idx);
                    if idx == next_idx {
                        tools.push(ToolAccumulator {
                            id,
                            name,
                            arguments: String::new(),
                        });
                    }
                }
                StreamChunk::ToolCallDelta {
                    id,
                    arguments_delta,
                } => {
                    if let Some(idx) = tool_index_by_id.get(&id).copied() {
                        tools[idx].arguments.push_str(&arguments_delta);
                    } else {
                        let idx = tools.len();
                        tool_index_by_id.insert(id.clone(), idx);
                        tools.push(ToolAccumulator {
                            id,
                            name: "tool".to_string(),
                            arguments: arguments_delta,
                        });
                    }
                }
                StreamChunk::ToolCallEnd { .. } => {}
                StreamChunk::Done { usage: done_usage } => {
                    if let Some(done_usage) = done_usage {
                        usage = done_usage;
                    }
                }
                StreamChunk::Error(message) => anyhow::bail!(message),
            }
        }

        let mut content = Vec::new();
        if !text.is_empty() {
            content.push(ContentPart::Text { text });
        }
        for tool in tools {
            content.push(ContentPart::ToolCall {
                id: tool.id,
                name: tool.name,
                arguments: tool.arguments,
                thought_signature: None,
            });
        }

        let finish_reason = if content
            .iter()
            .any(|part| matches!(part, ContentPart::ToolCall { .. }))
        {
            FinishReason::ToolCalls
        } else {
            FinishReason::Stop
        };

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage,
            finish_reason,
        })
    }

    async fn complete_stream_with_chatgpt_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let account_id = self.resolved_chatgpt_account_id(&access_token).context(
            "OpenAI Codex OAuth token is missing ChatGPT workspace/account ID. Re-run `codetether auth codex --device-code`.",
        )?;

        let (model, reasoning_effort) = Self::resolve_model_and_reasoning_effort(&request.model);
        let instructions = Self::extract_responses_instructions(&request.messages);
        let input = Self::convert_messages_to_responses_input(&request.messages);
        let tools = Self::convert_responses_tools(&request.tools);

        let mut body = json!({
            "model": model,
            "instructions": instructions,
            "input": input,
            "stream": true,
            "store": false,
            "tool_choice": "auto",
            "parallel_tool_calls": true,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(level) = reasoning_effort {
            body["reasoning"] = json!({ "effort": level.as_str() });
        }
        tracing::info!(
            backend = "chatgpt-codex",
            instructions_len = body
                .get("instructions")
                .and_then(|v| v.as_str())
                .map(str::len)
                .unwrap_or(0),
            input_items = body
                .get("input")
                .and_then(|v| v.as_array())
                .map(Vec::len)
                .unwrap_or(0),
            has_tools = !tools.is_empty(),
            "Sending responses request"
        );

        let response = self
            .client
            .post(format!("{}/responses", CHATGPT_CODEX_API_URL))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("chatgpt-account-id", account_id)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to ChatGPT Codex backend")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!(Self::format_openai_api_error(status, &body, &request.model));
        }

        let stream = response.bytes_stream().flat_map(|result| match result {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let mut chunks = Vec::new();

                for line in text.lines() {
                    if !line.starts_with("data: ") {
                        continue;
                    }
                    let data = &line[6..];
                    if data == "[DONE]" {
                        chunks.push(StreamChunk::Done { usage: None });
                        continue;
                    }

                    let Ok(event): Result<Value, _> = serde_json::from_str(data) else {
                        continue;
                    };
                    match event.get("type").and_then(Value::as_str) {
                        Some("response.output_text.delta") => {
                            if let Some(delta) = event.get("delta").and_then(Value::as_str) {
                                chunks.push(StreamChunk::Text(delta.to_string()));
                            }
                        }
                        Some("response.output_item.done") => {
                            if let Some(item) = event.get("item")
                                && item.get("type").and_then(Value::as_str) == Some("function_call")
                            {
                                let call_id = item
                                    .get("call_id")
                                    .or_else(|| item.get("id"))
                                    .and_then(Value::as_str);
                                let name = item.get("name").and_then(Value::as_str);
                                let arguments = item.get("arguments").and_then(Value::as_str);

                                if let (Some(call_id), Some(name)) = (call_id, name) {
                                    chunks.push(StreamChunk::ToolCallStart {
                                        id: call_id.to_string(),
                                        name: name.to_string(),
                                    });
                                    if let Some(arguments) = arguments {
                                        chunks.push(StreamChunk::ToolCallDelta {
                                            id: call_id.to_string(),
                                            arguments_delta: arguments.to_string(),
                                        });
                                    }
                                }
                            }
                        }
                        Some("response.completed") | Some("response.done") => {
                            let usage = event
                                .get("response")
                                .and_then(|response| response.get("usage"))
                                .map(|usage| Self::parse_responses_usage(Some(usage)));
                            chunks.push(StreamChunk::Done { usage });
                        }
                        Some("response.failed") => {
                            let message = event
                                .get("response")
                                .and_then(|response| response.get("error"))
                                .and_then(|error| error.get("message"))
                                .and_then(Value::as_str)
                                .unwrap_or("Response failed")
                                .to_string();
                            chunks.push(StreamChunk::Error(message));
                        }
                        _ => {}
                    }
                }

                futures::stream::iter(chunks)
            }
            Err(e) => futures::stream::iter(vec![StreamChunk::Error(e.to_string())]),
        });

        Ok(Box::pin(stream))
    }

    async fn complete_stream_with_openai_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let (model, reasoning_effort) = Self::resolve_model_and_reasoning_effort(&request.model);
        let instructions = Self::extract_responses_instructions(&request.messages);
        let input = Self::convert_messages_to_responses_input(&request.messages);
        let tools = Self::convert_responses_tools(&request.tools);

        let mut body = json!({
            "model": model,
            "instructions": instructions,
            "input": input,
            "stream": true,
            "store": false,
            "tool_choice": "auto",
            "parallel_tool_calls": true,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(level) = reasoning_effort {
            body["reasoning"] = json!({ "effort": level.as_str() });
        }
        tracing::info!(
            backend = "openai-responses",
            instructions_len = body
                .get("instructions")
                .and_then(|v| v.as_str())
                .map(str::len)
                .unwrap_or(0),
            input_items = body
                .get("input")
                .and_then(|v| v.as_array())
                .map(Vec::len)
                .unwrap_or(0),
            has_tools = !tools.is_empty(),
            "Sending responses request"
        );

        let response = self
            .client
            .post(format!("{}/responses", OPENAI_API_URL))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to OpenAI responses API")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!(Self::format_openai_api_error(status, &body, &request.model));
        }

        let stream = response.bytes_stream().flat_map(|result| match result {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let mut chunks = Vec::new();

                for line in text.lines() {
                    if !line.starts_with("data: ") {
                        continue;
                    }
                    let data = &line[6..];
                    if data == "[DONE]" {
                        chunks.push(StreamChunk::Done { usage: None });
                        continue;
                    }

                    let Ok(event): Result<Value, _> = serde_json::from_str(data) else {
                        continue;
                    };
                    match event.get("type").and_then(Value::as_str) {
                        Some("response.output_text.delta") => {
                            if let Some(delta) = event.get("delta").and_then(Value::as_str) {
                                chunks.push(StreamChunk::Text(delta.to_string()));
                            }
                        }
                        Some("response.output_item.done") => {
                            if let Some(item) = event.get("item")
                                && item.get("type").and_then(Value::as_str) == Some("function_call")
                            {
                                let call_id = item
                                    .get("call_id")
                                    .or_else(|| item.get("id"))
                                    .and_then(Value::as_str);
                                let name = item.get("name").and_then(Value::as_str);
                                let arguments = item.get("arguments").and_then(Value::as_str);

                                if let (Some(call_id), Some(name)) = (call_id, name) {
                                    chunks.push(StreamChunk::ToolCallStart {
                                        id: call_id.to_string(),
                                        name: name.to_string(),
                                    });
                                    if let Some(arguments) = arguments {
                                        chunks.push(StreamChunk::ToolCallDelta {
                                            id: call_id.to_string(),
                                            arguments_delta: arguments.to_string(),
                                        });
                                    }
                                }
                            }
                        }
                        Some("response.completed") | Some("response.done") => {
                            let usage = event
                                .get("response")
                                .and_then(|response| response.get("usage"))
                                .map(|usage| Self::parse_responses_usage(Some(usage)));
                            chunks.push(StreamChunk::Done { usage });
                        }
                        Some("response.failed") => {
                            let message = event
                                .get("response")
                                .and_then(|response| response.get("error"))
                                .and_then(|error| error.get("message"))
                                .and_then(Value::as_str)
                                .unwrap_or("Response failed")
                                .to_string();
                            chunks.push(StreamChunk::Error(message));
                        }
                        _ => {}
                    }
                }

                futures::stream::iter(chunks)
            }
            Err(e) => futures::stream::iter(vec![StreamChunk::Error(e.to_string())]),
        });

        Ok(Box::pin(stream))
    }
}

#[async_trait]
impl Provider for OpenAiCodexProvider {
    fn name(&self) -> &str {
        "openai-codex"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "gpt-5".to_string(),
                name: "GPT-5".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 400_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "gpt-5-mini".to_string(),
                name: "GPT-5 Mini".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 264_000,
                max_output_tokens: Some(64_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "gpt-5.1-codex".to_string(),
                name: "GPT-5.1 Codex".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 400_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "gpt-5.2".to_string(),
                name: "GPT-5.2".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 400_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "gpt-5.3-codex".to_string(),
                name: "GPT-5.3 Codex".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 400_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "o3".to_string(),
                name: "O3".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(100_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "o4-mini".to_string(),
                name: "O4 Mini".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(100_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let access_token = self.get_access_token().await?;
        if self.using_chatgpt_backend() {
            return self
                .complete_with_chatgpt_responses(request, access_token)
                .await;
        }
        self.complete_with_openai_responses(request, access_token)
            .await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let access_token = self.get_access_token().await?;
        if self.using_chatgpt_backend() {
            return self
                .complete_stream_with_chatgpt_responses(request, access_token)
                .await;
        }
        self.complete_stream_with_openai_responses(request, access_token)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pkce() {
        let pkce = OpenAiCodexProvider::generate_pkce();
        assert!(!pkce.verifier.is_empty());
        assert!(!pkce.challenge.is_empty());
        assert_ne!(pkce.verifier, pkce.challenge);
    }

    #[test]
    fn test_generate_state() {
        let state = OpenAiCodexProvider::generate_state();
        assert_eq!(state.len(), 32);
    }

    #[test]
    fn formats_scope_error_with_actionable_message() {
        let body = r#"{"error":{"message":"Missing scopes: model.request"}}"#;
        let msg = OpenAiCodexProvider::format_openai_api_error(
            StatusCode::UNAUTHORIZED,
            body,
            "gpt-5.3-codex",
        );
        assert!(msg.contains("model.request"));
        assert!(msg.contains("codetether auth codex"));
    }

    #[test]
    fn parses_model_suffix_for_thinking_level() {
        let (model, level) =
            OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.3-codex:high");
        assert_eq!(model, "gpt-5.3-codex");
        assert_eq!(level.map(ThinkingLevel::as_str), Some("high"));
    }

    #[test]
    fn ignores_unknown_model_suffix() {
        let (model, level) =
            OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.3-codex:turbo");
        assert_eq!(model, "gpt-5.3-codex:turbo");
        assert_eq!(level, None);
    }

    #[test]
    fn skips_reasoning_effort_for_non_reasoning_models() {
        let (model, level) = OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-4o:high");
        assert_eq!(model, "gpt-4o");
        assert_eq!(level, None);
    }

    #[test]
    fn extracts_chatgpt_account_id_from_jwt_claims() {
        let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"none","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD
            .encode(r#"{"https://api.openai.com/auth":{"chatgpt_account_id":"org_test123"}}"#);
        let jwt = format!("{header}.{payload}.sig");

        let account_id = OpenAiCodexProvider::extract_chatgpt_account_id(&jwt);
        assert_eq!(account_id.as_deref(), Some("org_test123"));
    }

    #[test]
    fn extracts_chatgpt_account_id_from_organizations_claim() {
        let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"none","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD.encode(r#"{"organizations":[{"id":"org_from_list"}]}"#);
        let jwt = format!("{header}.{payload}.sig");

        let account_id = OpenAiCodexProvider::extract_chatgpt_account_id(&jwt);
        assert_eq!(account_id.as_deref(), Some("org_from_list"));
    }

    #[test]
    fn extracts_responses_instructions_from_system_messages() {
        let messages = vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "first system block".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "user message".to_string(),
                }],
            },
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "second system block".to_string(),
                }],
            },
        ];

        let instructions = OpenAiCodexProvider::extract_responses_instructions(&messages);
        assert_eq!(instructions, "first system block\n\nsecond system block");
    }

    #[test]
    fn falls_back_to_default_responses_instructions_without_system_message() {
        let messages = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "only user".to_string(),
            }],
        }];

        let instructions = OpenAiCodexProvider::extract_responses_instructions(&messages);
        assert_eq!(instructions, DEFAULT_RESPONSES_INSTRUCTIONS);
    }

    #[test]
    fn responses_input_ignores_system_messages() {
        let messages = vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "system".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "user".to_string(),
                }],
            },
        ];

        let input = OpenAiCodexProvider::convert_messages_to_responses_input(&messages);
        assert_eq!(input.len(), 1);
        assert_eq!(input[0].get("role").and_then(Value::as_str), Some("user"));
    }
}
