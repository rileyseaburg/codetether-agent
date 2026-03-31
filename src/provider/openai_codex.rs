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
use futures::stream::BoxStream;
use futures::{SinkExt, StreamExt};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{
        Error as WsError, Message as WsMessage,
        client::IntoClientRequest,
        http::{HeaderValue, Request},
    },
};

const OPENAI_API_URL: &str = "https://api.openai.com/v1";
const CHATGPT_CODEX_API_URL: &str = "https://chatgpt.com/backend-api/codex";
const OPENAI_RESPONSES_WS_URL: &str = "wss://api.openai.com/v1/responses";
const CHATGPT_CODEX_RESPONSES_WS_URL: &str = "wss://chatgpt.com/backend-api/codex/responses";
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResponsesWsBackend {
    OpenAi,
    ChatGptCodex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CodexServiceTier {
    Priority,
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

impl CodexServiceTier {
    fn as_str(self) -> &'static str {
        match self {
            Self::Priority => "priority",
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

#[derive(Debug, Default)]
struct ResponsesToolState {
    call_id: String,
    name: Option<String>,
    started: bool,
    finished: bool,
    emitted_arguments: String,
}

#[derive(Debug, Default)]
struct ResponsesSseParser {
    line_buffer: String,
    event_data_lines: Vec<String>,
    tools: HashMap<String, ResponsesToolState>,
}

pub struct OpenAiRealtimeConnection<S = MaybeTlsStream<TcpStream>> {
    stream: WebSocketStream<S>,
}

impl<S> std::fmt::Debug for OpenAiRealtimeConnection<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiRealtimeConnection").finish()
    }
}

impl<S> OpenAiRealtimeConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    fn new(stream: WebSocketStream<S>) -> Self {
        Self { stream }
    }

    pub async fn send_event(&mut self, event: &Value) -> Result<()> {
        let payload = serde_json::to_string(event).context("Failed to serialize Realtime event")?;
        self.stream
            .send(WsMessage::Text(payload.into()))
            .await
            .context("Failed to send Realtime event")?;
        Ok(())
    }

    pub async fn recv_event(&mut self) -> Result<Option<Value>> {
        while let Some(message) = self.stream.next().await {
            let message = message.context("Realtime WebSocket receive failed")?;
            match message {
                WsMessage::Text(text) => {
                    let event = serde_json::from_str(&text)
                        .context("Failed to parse Realtime text event")?;
                    return Ok(Some(event));
                }
                WsMessage::Binary(bytes) => {
                    let text = String::from_utf8(bytes.to_vec())
                        .context("Realtime binary event was not valid UTF-8")?;
                    let event = serde_json::from_str(&text)
                        .context("Failed to parse Realtime binary event")?;
                    return Ok(Some(event));
                }
                WsMessage::Ping(payload) => {
                    self.stream
                        .send(WsMessage::Pong(payload))
                        .await
                        .context("Failed to respond to Realtime ping")?;
                }
                WsMessage::Pong(_) => {}
                WsMessage::Frame(_) => {}
                WsMessage::Close(_) => return Ok(None),
            }
        }

        Ok(None)
    }

    pub async fn close(&mut self) -> Result<()> {
        match self.stream.send(WsMessage::Close(None)).await {
            Ok(()) => {}
            Err(WsError::ConnectionClosed) | Err(WsError::AlreadyClosed) => {}
            Err(WsError::Io(err))
                if matches!(
                    err.kind(),
                    ErrorKind::BrokenPipe
                        | ErrorKind::ConnectionReset
                        | ErrorKind::ConnectionAborted
                        | ErrorKind::NotConnected
                ) => {}
            Err(err) => return Err(err).context("Failed to close Realtime WebSocket"),
        }
        Ok(())
    }
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

impl Default for OpenAiCodexProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAiCodexProvider {
    fn chatgpt_supported_models() -> &'static [&'static str] {
        &[
            "gpt-5",
            "gpt-5-mini",
            "gpt-5.1-codex",
            "gpt-5.2",
            "gpt-5.3-codex",
            "gpt-5.4",
            "gpt-5.4-fast",
            "o3",
            "o4-mini",
        ]
    }

    fn model_is_supported_by_backend(&self, model: &str) -> bool {
        if !self.using_chatgpt_backend() {
            return true;
        }
        Self::chatgpt_supported_models().contains(&model)
    }

    fn validate_model_for_backend(&self, model: &str) -> Result<()> {
        let (resolved_model, _, _) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(model);
        if self.model_is_supported_by_backend(model)
            || self.model_is_supported_by_backend(&resolved_model)
        {
            return Ok(());
        }

        if self.using_chatgpt_backend() {
            anyhow::bail!(
                "Model '{}' is not supported when using Codex with a ChatGPT account. Supported models: {}",
                model,
                Self::chatgpt_supported_models().join(", ")
            );
        }

        Ok(())
    }

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

    fn responses_ws_url_with_base(base_url: &str) -> String {
        base_url.to_string()
    }

    pub fn responses_ws_url() -> String {
        Self::responses_ws_url_with_base(OPENAI_RESPONSES_WS_URL)
    }

    fn build_responses_ws_request_with_base_url_and_account_id(
        base_url: &str,
        token: &str,
        chatgpt_account_id: Option<&str>,
    ) -> Result<Request<()>> {
        if token.trim().is_empty() {
            anyhow::bail!("Responses WebSocket token cannot be empty");
        }

        let url = Self::responses_ws_url_with_base(base_url);
        let mut request = url
            .into_client_request()
            .context("Failed to build Responses WebSocket request")?;
        request.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {token}"))
                .context("Failed to build Responses Authorization header")?,
        );
        request.headers_mut().insert(
            "User-Agent",
            HeaderValue::from_static("codetether-responses-ws/1.0"),
        );
        if let Some(account_id) = chatgpt_account_id.filter(|id| !id.trim().is_empty()) {
            request.headers_mut().insert(
                "ChatGPT-Account-ID",
                HeaderValue::from_str(account_id)
                    .context("Failed to build ChatGPT account header")?,
            );
        }
        Ok(request)
    }

    fn build_responses_ws_request_with_base_url(
        base_url: &str,
        token: &str,
    ) -> Result<Request<()>> {
        Self::build_responses_ws_request_with_base_url_and_account_id(base_url, token, None)
    }

    fn build_responses_ws_request_with_account_id(
        token: &str,
        chatgpt_account_id: Option<&str>,
    ) -> Result<Request<()>> {
        Self::build_responses_ws_request_with_base_url_and_account_id(
            OPENAI_RESPONSES_WS_URL,
            token,
            chatgpt_account_id,
        )
    }

    fn build_chatgpt_responses_ws_request(
        token: &str,
        chatgpt_account_id: Option<&str>,
    ) -> Result<Request<()>> {
        Self::build_responses_ws_request_with_base_url_and_account_id(
            CHATGPT_CODEX_RESPONSES_WS_URL,
            token,
            chatgpt_account_id,
        )
    }

    async fn connect_responses_ws_with_token(
        &self,
        token: &str,
        chatgpt_account_id: Option<&str>,
        backend: ResponsesWsBackend,
    ) -> Result<OpenAiRealtimeConnection> {
        let request = match backend {
            ResponsesWsBackend::OpenAi => {
                Self::build_responses_ws_request_with_account_id(token, chatgpt_account_id)?
            }
            ResponsesWsBackend::ChatGptCodex => {
                Self::build_chatgpt_responses_ws_request(token, chatgpt_account_id)?
            }
        };
        let (stream, _response) = connect_async(request)
            .await
            .context("Failed to connect to OpenAI Responses WebSocket")?;
        Ok(OpenAiRealtimeConnection::new(stream))
    }

    pub async fn connect_responses_ws(&self) -> Result<OpenAiRealtimeConnection> {
        let token = self.get_access_token().await?;
        let account_id = self.resolved_chatgpt_account_id(&token);
        let backend = if self.using_chatgpt_backend() {
            ResponsesWsBackend::ChatGptCodex
        } else {
            ResponsesWsBackend::OpenAi
        };
        self.connect_responses_ws_with_token(&token, account_id.as_deref(), backend)
            .await
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
        let mut known_tool_call_ids = std::collections::HashSet::new();

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
                            known_tool_call_ids.insert(id.clone());
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
                            if known_tool_call_ids.contains(tool_call_id) {
                                input.push(json!({
                                    "type": "function_call_output",
                                    "call_id": tool_call_id,
                                    "output": content,
                                }));
                            } else {
                                tracing::warn!(
                                    tool_call_id = %tool_call_id,
                                    "Skipping orphaned function_call_output while building Codex responses input"
                                );
                            }
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

    fn extract_json_string(value: Option<&Value>) -> Option<String> {
        match value {
            Some(Value::String(text)) => Some(text.clone()),
            Some(other) => serde_json::to_string(other).ok(),
            None => None,
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

    fn parse_service_tier_model_alias(model: &str) -> (String, Option<CodexServiceTier>) {
        match model {
            // OpenAI's Codex app implements GPT-5.4 Fast mode via `service_tier=priority`.
            "gpt-5.4-fast" => ("gpt-5.4".to_string(), Some(CodexServiceTier::Priority)),
            _ => (model.to_string(), None),
        }
    }

    fn resolve_model_and_reasoning_effort(model: &str) -> (String, Option<ThinkingLevel>) {
        let (base_model, level, _) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(model);
        (base_model, level)
    }

    fn resolve_model_and_reasoning_effort_and_service_tier(
        model: &str,
    ) -> (String, Option<ThinkingLevel>, Option<CodexServiceTier>) {
        let (base_model, level_from_model) = Self::parse_model_thinking_level(model);
        let level = level_from_model.or_else(Self::env_thinking_level);
        let (base_model, service_tier) = Self::parse_service_tier_model_alias(&base_model);
        if !Self::model_supports_reasoning_effort(&base_model) {
            return (base_model, None, service_tier);
        }
        (base_model, level, service_tier)
    }

    fn apply_service_tier(payload: &mut Value, service_tier: Option<CodexServiceTier>) {
        if let Some(service_tier) = service_tier {
            payload["service_tier"] = json!(service_tier.as_str());
        }
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

    fn build_responses_ws_create_event(
        request: &CompletionRequest,
        model: &str,
        reasoning_effort: Option<ThinkingLevel>,
        service_tier: Option<CodexServiceTier>,
    ) -> Value {
        Self::build_responses_ws_create_event_for_backend(
            request,
            model,
            reasoning_effort,
            service_tier,
            ResponsesWsBackend::OpenAi,
        )
    }

    fn build_responses_ws_create_event_for_backend(
        request: &CompletionRequest,
        model: &str,
        reasoning_effort: Option<ThinkingLevel>,
        service_tier: Option<CodexServiceTier>,
        backend: ResponsesWsBackend,
    ) -> Value {
        let instructions = Self::extract_responses_instructions(&request.messages);
        let input = Self::convert_messages_to_responses_input(&request.messages);
        let tools = Self::convert_responses_tools(&request.tools);

        let mut event = json!({
            "type": "response.create",
            "model": model,
            "store": false,
            "instructions": instructions,
            "input": input,
        });

        if !tools.is_empty() {
            event["tools"] = json!(tools);
        }
        if let Some(level) = reasoning_effort {
            event["reasoning"] = json!({ "effort": level.as_str() });
        }
        Self::apply_service_tier(&mut event, service_tier);
        if backend == ResponsesWsBackend::OpenAi {
            event["tool_choice"] = json!("auto");
            event["parallel_tool_calls"] = json!(true);
        }
        if backend == ResponsesWsBackend::OpenAi
            && let Some(max_tokens) = request.max_tokens
        {
            event["max_output_tokens"] = json!(max_tokens);
        }

        event
    }

    fn finish_responses_tool_call(
        parser: &mut ResponsesSseParser,
        key: &str,
        chunks: &mut Vec<StreamChunk>,
    ) {
        if let Some(state) = parser.tools.get_mut(key)
            && !state.finished
        {
            chunks.push(StreamChunk::ToolCallEnd {
                id: state.call_id.clone(),
            });
            state.finished = true;
        }
    }

    fn parse_responses_event(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
    ) {
        match event.get("type").and_then(Value::as_str) {
            Some("response.output_text.delta") => {
                if let Some(delta) = event.get("delta").and_then(Value::as_str) {
                    chunks.push(StreamChunk::Text(delta.to_string()));
                }
            }
            Some("response.output_item.added") => {
                if let Some(item) = event.get("item") {
                    Self::record_responses_tool_item(parser, item, chunks, false);
                }
            }
            Some("response.function_call_arguments.delta") => {
                Self::record_responses_tool_arguments(parser, &event, chunks, false);
            }
            Some("response.function_call_arguments.done") => {
                Self::record_responses_tool_arguments(parser, &event, chunks, true);
            }
            Some("response.output_item.done") => {
                if let Some(item) = event.get("item")
                    && item.get("type").and_then(Value::as_str) == Some("function_call")
                    && let Some(key) = item
                        .get("id")
                        .and_then(Value::as_str)
                        .or_else(|| item.get("call_id").and_then(Value::as_str))
                {
                    Self::record_responses_tool_item(parser, item, chunks, true);
                    Self::finish_responses_tool_call(parser, key, chunks);
                }
            }
            Some("response.completed") | Some("response.done") => {
                if let Some(output) = event
                    .get("response")
                    .and_then(|response| response.get("output"))
                    .and_then(Value::as_array)
                {
                    for item in output {
                        if item.get("type").and_then(Value::as_str) == Some("function_call")
                            && let Some(key) = item
                                .get("id")
                                .and_then(Value::as_str)
                                .or_else(|| item.get("call_id").and_then(Value::as_str))
                        {
                            Self::record_responses_tool_item(parser, item, chunks, true);
                            Self::finish_responses_tool_call(parser, key, chunks);
                        }
                    }
                }
                let failed_message = event
                    .get("response")
                    .and_then(|response| response.get("status"))
                    .and_then(Value::as_str)
                    .filter(|status| matches!(*status, "failed" | "cancelled" | "incomplete"))
                    .map(|_| {
                        event
                            .get("response")
                            .and_then(|response| response.get("error"))
                            .and_then(|error| error.get("message"))
                            .and_then(Value::as_str)
                            .unwrap_or("Response failed")
                            .to_string()
                    });
                if let Some(message) = failed_message {
                    chunks.push(StreamChunk::Error(message));
                    return;
                }
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
                    .or_else(|| event.get("error").and_then(|error| error.get("message")))
                    .and_then(Value::as_str)
                    .unwrap_or("Response failed")
                    .to_string();
                chunks.push(StreamChunk::Error(message));
            }
            Some("error") => {
                let message = event
                    .get("error")
                    .and_then(|error| error.get("message"))
                    .or_else(|| event.get("message"))
                    .and_then(Value::as_str)
                    .unwrap_or("Realtime error")
                    .to_string();
                chunks.push(StreamChunk::Error(message));
            }
            _ => {}
        }
    }

    fn parse_responses_sse_event(
        parser: &mut ResponsesSseParser,
        data: &str,
        chunks: &mut Vec<StreamChunk>,
    ) {
        if data == "[DONE]" {
            chunks.push(StreamChunk::Done { usage: None });
            return;
        }

        let Ok(event): Result<Value, _> = serde_json::from_str(data) else {
            return;
        };

        Self::parse_responses_event(parser, &event, chunks);
    }

    fn record_responses_tool_item(
        parser: &mut ResponsesSseParser,
        item: &Value,
        chunks: &mut Vec<StreamChunk>,
        include_arguments: bool,
    ) -> Option<String> {
        if item.get("type").and_then(Value::as_str) != Some("function_call") {
            return None;
        }

        let key = item
            .get("id")
            .and_then(Value::as_str)
            .or_else(|| item.get("call_id").and_then(Value::as_str))?;
        let call_id = item
            .get("call_id")
            .and_then(Value::as_str)
            .or_else(|| item.get("id").and_then(Value::as_str))?
            .to_string();
        let name = item.get("name").and_then(Value::as_str).map(str::to_string);
        let arguments = if include_arguments {
            Self::extract_json_string(item.get("arguments"))
        } else {
            None
        };

        let state = parser
            .tools
            .entry(key.to_string())
            .or_insert_with(|| ResponsesToolState {
                call_id: call_id.clone(),
                ..ResponsesToolState::default()
            });
        if state.call_id.is_empty() {
            state.call_id = call_id.clone();
        }
        if state.name.is_none() {
            state.name = name;
        }
        if !state.started
            && let Some(name) = state.name.clone()
        {
            chunks.push(StreamChunk::ToolCallStart {
                id: state.call_id.clone(),
                name,
            });
            state.started = true;
        }
        if let Some(arguments) = arguments {
            Self::emit_missing_responses_tool_arguments(state, arguments, chunks);
        }

        Some(state.call_id.clone())
    }

    fn record_responses_tool_arguments(
        parser: &mut ResponsesSseParser,
        event: &Value,
        chunks: &mut Vec<StreamChunk>,
        use_final_arguments: bool,
    ) {
        let key = event
            .get("item_id")
            .and_then(Value::as_str)
            .or_else(|| event.get("call_id").and_then(Value::as_str));
        let Some(key) = key else {
            return;
        };

        let fallback_call_id = event
            .get("call_id")
            .and_then(Value::as_str)
            .unwrap_or(key)
            .to_string();
        let state = parser
            .tools
            .entry(key.to_string())
            .or_insert_with(|| ResponsesToolState {
                call_id: fallback_call_id.clone(),
                ..ResponsesToolState::default()
            });
        if state.call_id.is_empty() {
            state.call_id = fallback_call_id;
        }

        if !state.started
            && let Some(name) = event.get("name").and_then(Value::as_str)
        {
            state.name = Some(name.to_string());
            chunks.push(StreamChunk::ToolCallStart {
                id: state.call_id.clone(),
                name: name.to_string(),
            });
            state.started = true;
        }

        let arguments = if use_final_arguments {
            Self::extract_json_string(event.get("arguments")).or_else(|| {
                event
                    .get("delta")
                    .and_then(Value::as_str)
                    .map(str::to_string)
            })
        } else {
            event
                .get("delta")
                .and_then(Value::as_str)
                .map(str::to_string)
        };

        if let Some(arguments) = arguments {
            Self::emit_missing_responses_tool_arguments(state, arguments, chunks);
        }
    }

    fn emit_missing_responses_tool_arguments(
        state: &mut ResponsesToolState,
        arguments: String,
        chunks: &mut Vec<StreamChunk>,
    ) {
        let delta = if arguments.starts_with(&state.emitted_arguments) {
            arguments[state.emitted_arguments.len()..].to_string()
        } else if state.emitted_arguments.is_empty() {
            arguments.clone()
        } else if arguments == state.emitted_arguments {
            String::new()
        } else {
            arguments.clone()
        };

        if !delta.is_empty() {
            chunks.push(StreamChunk::ToolCallDelta {
                id: state.call_id.clone(),
                arguments_delta: delta.clone(),
            });
            state.emitted_arguments.push_str(&delta);
        }
    }

    fn parse_responses_sse_bytes(
        parser: &mut ResponsesSseParser,
        bytes: &[u8],
    ) -> Vec<StreamChunk> {
        parser.line_buffer.push_str(&String::from_utf8_lossy(bytes));
        let mut chunks = Vec::new();

        while let Some(line_end) = parser.line_buffer.find('\n') {
            let mut line = parser.line_buffer[..line_end].to_string();
            parser.line_buffer.drain(..=line_end);

            if line.ends_with('\r') {
                line.pop();
            }

            if line.is_empty() {
                if !parser.event_data_lines.is_empty() {
                    let data = parser.event_data_lines.join("\n");
                    parser.event_data_lines.clear();
                    Self::parse_responses_sse_event(parser, &data, &mut chunks);
                }
                continue;
            }

            if let Some(data) = line.strip_prefix("data:") {
                let data = data.strip_prefix(' ').unwrap_or(data);
                parser.event_data_lines.push(data.to_string());
            }
        }

        chunks
    }

    fn finish_responses_sse_parser(parser: &mut ResponsesSseParser) -> Vec<StreamChunk> {
        let mut chunks = Vec::new();

        if !parser.line_buffer.is_empty() {
            let mut line = std::mem::take(&mut parser.line_buffer);
            if line.ends_with('\r') {
                line.pop();
            }
            if let Some(data) = line.strip_prefix("data:") {
                let data = data.strip_prefix(' ').unwrap_or(data);
                parser.event_data_lines.push(data.to_string());
            }
        }

        if !parser.event_data_lines.is_empty() {
            let data = parser.event_data_lines.join("\n");
            parser.event_data_lines.clear();
            Self::parse_responses_sse_event(parser, &data, &mut chunks);
        }

        chunks
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
                    } else if tools[idx].name == "tool" {
                        tools[idx].name = name;
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
        match self
            .complete_stream_with_realtime(
                request.clone(),
                access_token.clone(),
                Some(account_id.clone()),
                "chatgpt-codex-responses-ws",
                ResponsesWsBackend::ChatGptCodex,
            )
            .await
        {
            Ok(stream) => Ok(stream),
            Err(error) => {
                tracing::warn!(
                    error = %error,
                    "Responses WebSocket connect failed for ChatGPT Codex backend; falling back to HTTP responses streaming"
                );
                self.complete_stream_with_chatgpt_http_responses(request, access_token, account_id)
                    .await
            }
        }
    }

    async fn complete_stream_with_openai_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        match self
            .complete_stream_with_realtime(
                request.clone(),
                api_key.clone(),
                None,
                "openai-responses-ws",
                ResponsesWsBackend::OpenAi,
            )
            .await
        {
            Ok(stream) => Ok(stream),
            Err(error) => {
                tracing::warn!(
                    error = %error,
                    "Responses WebSocket connect failed for OpenAI backend; falling back to HTTP responses streaming"
                );
                self.complete_stream_with_openai_http_responses(request, api_key)
                    .await
            }
        }
    }

    async fn complete_stream_with_chatgpt_http_responses(
        &self,
        request: CompletionRequest,
        access_token: String,
        account_id: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let (model, reasoning_effort, service_tier) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(&request.model);
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
        Self::apply_service_tier(&mut body, service_tier);

        tracing::info!(
            backend = "chatgpt-codex-responses-http",
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
            "Sending HTTP responses request"
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

        let stream = async_stream::stream! {
            let mut parser = ResponsesSseParser::default();
            let mut byte_stream = response.bytes_stream();

            while let Some(result) = byte_stream.next().await {
                match result {
                    Ok(bytes) => {
                        for chunk in Self::parse_responses_sse_bytes(&mut parser, &bytes) {
                            yield chunk;
                        }
                    }
                    Err(error) => yield StreamChunk::Error(error.to_string()),
                }
            }

            for chunk in Self::finish_responses_sse_parser(&mut parser) {
                yield chunk;
            }
        };

        Ok(Box::pin(stream))
    }

    async fn complete_stream_with_openai_http_responses(
        &self,
        request: CompletionRequest,
        api_key: String,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let (model, reasoning_effort, service_tier) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(&request.model);
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
        Self::apply_service_tier(&mut body, service_tier);

        tracing::info!(
            backend = "openai-responses-http",
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
            "Sending HTTP responses request"
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

        let stream = async_stream::stream! {
            let mut parser = ResponsesSseParser::default();
            let mut byte_stream = response.bytes_stream();

            while let Some(result) = byte_stream.next().await {
                match result {
                    Ok(bytes) => {
                        for chunk in Self::parse_responses_sse_bytes(&mut parser, &bytes) {
                            yield chunk;
                        }
                    }
                    Err(error) => yield StreamChunk::Error(error.to_string()),
                }
            }

            for chunk in Self::finish_responses_sse_parser(&mut parser) {
                yield chunk;
            }
        };

        Ok(Box::pin(stream))
    }

    async fn complete_stream_with_realtime(
        &self,
        request: CompletionRequest,
        access_token: String,
        chatgpt_account_id: Option<String>,
        backend: &'static str,
        ws_backend: ResponsesWsBackend,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let (model, reasoning_effort, service_tier) =
            Self::resolve_model_and_reasoning_effort_and_service_tier(&request.model);
        let body = Self::build_responses_ws_create_event_for_backend(
            &request,
            &model,
            reasoning_effort,
            service_tier,
            ws_backend,
        );
        tracing::info!(
            backend = backend,
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
            has_tools = body
                .get("tools")
                .and_then(|v| v.as_array())
                .is_some_and(|tools| !tools.is_empty()),
            "Sending responses websocket request"
        );

        let connection = self
            .connect_responses_ws_with_token(
                &access_token,
                chatgpt_account_id.as_deref(),
                ws_backend,
            )
            .await?;

        let stream = async_stream::stream! {
            let mut connection = connection;
            let mut parser = ResponsesSseParser::default();
            if let Err(error) = connection.send_event(&body).await {
                yield StreamChunk::Error(error.to_string());
                let _ = connection.close().await;
                return;
            }

            let mut saw_terminal = false;
            loop {
                match connection.recv_event().await {
                    Ok(Some(event)) => {
                        let mut chunks = Vec::new();
                        Self::parse_responses_event(&mut parser, &event, &mut chunks);
                        for chunk in chunks {
                            if matches!(chunk, StreamChunk::Done { .. } | StreamChunk::Error(_)) {
                                saw_terminal = true;
                            }
                            yield chunk;
                        }
                        if saw_terminal {
                            break;
                        }
                    }
                    Ok(None) => {
                        if !saw_terminal {
                            yield StreamChunk::Error("Realtime WebSocket closed before response completion".to_string());
                        }
                        break;
                    }
                    Err(error) => {
                        yield StreamChunk::Error(error.to_string());
                        break;
                    }
                }
            }

            let _ = connection.close().await;
        };

        Ok(Box::pin(stream))
    }
}

#[async_trait]
impl Provider for OpenAiCodexProvider {
    fn name(&self) -> &str {
        "openai-codex"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let mut models = vec![
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
                id: "gpt-5.4".to_string(),
                name: "GPT-5.4".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 272_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "gpt-5.4-fast".to_string(),
                name: "GPT-5.4 Fast".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 272_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0),
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "gpt-5.4-pro".to_string(),
                name: "GPT-5.4 Pro".to_string(),
                provider: "openai-codex".to_string(),
                context_window: 272_000,
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
        ];

        if self.using_chatgpt_backend() {
            models.retain(|model| Self::chatgpt_supported_models().contains(&model.id.as_str()));
        }

        Ok(models)
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.validate_model_for_backend(&request.model)?;
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
        self.validate_model_for_backend(&request.model)?;
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
    use futures::stream;
    use tokio::io::duplex;
    use tokio_tungstenite::{
        accept_hdr_async, client_async,
        tungstenite::{
            Message as WsMessage,
            handshake::server::{Request as ServerRequest, Response as ServerResponse},
        },
    };

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
    fn maps_fast_model_alias_to_priority_service_tier() {
        let (model, level, service_tier) =
            OpenAiCodexProvider::resolve_model_and_reasoning_effort_and_service_tier(
                "gpt-5.4-fast:high",
            );
        assert_eq!(model, "gpt-5.4");
        assert_eq!(level.map(ThinkingLevel::as_str), Some("high"));
        assert_eq!(service_tier.map(CodexServiceTier::as_str), Some("priority"));
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

    #[tokio::test]
    async fn lists_gpt_5_4_models() {
        let provider = OpenAiCodexProvider::new();
        let models = provider
            .list_models()
            .await
            .expect("model listing should succeed");

        assert!(models.iter().any(|model| model.id == "gpt-5.4"));
        assert!(models.iter().any(|model| model.id == "gpt-5.4-fast"));
        assert!(!models.iter().any(|model| model.id == "gpt-5.4-pro"));
    }

    #[test]
    fn rejects_pro_model_for_chatgpt_backend() {
        let provider = OpenAiCodexProvider::new();
        let err = provider
            .validate_model_for_backend("gpt-5.4-pro")
            .expect_err("chatgpt backend should reject unsupported model");
        assert!(
            err.to_string()
                .contains("not supported when using Codex with a ChatGPT account")
        );
    }

    #[test]
    fn allows_pro_model_for_api_key_backend() {
        let provider = OpenAiCodexProvider::from_api_key("test-key".to_string());
        provider
            .validate_model_for_backend("gpt-5.4-pro")
            .expect("api key backend should allow pro model");
    }

    #[test]
    fn allows_fast_alias_for_chatgpt_backend() {
        let provider = OpenAiCodexProvider::new();
        provider
            .validate_model_for_backend("gpt-5.4-fast:high")
            .expect("chatgpt backend should allow fast alias");
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

    #[test]
    fn responses_ws_request_uses_bearer_auth() {
        let request = OpenAiCodexProvider::build_responses_ws_request_with_base_url(
            "wss://example.com/v1/responses",
            "test-token",
        )
        .expect("request should build");

        assert_eq!(request.uri().to_string(), "wss://example.com/v1/responses");
        assert_eq!(
            request
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok()),
            Some("Bearer test-token")
        );
        assert_eq!(
            request
                .headers()
                .get("User-Agent")
                .and_then(|v| v.to_str().ok()),
            Some("codetether-responses-ws/1.0")
        );
    }

    #[test]
    fn responses_ws_request_includes_chatgpt_account_id_when_provided() {
        let request = OpenAiCodexProvider::build_responses_ws_request_with_base_url_and_account_id(
            "wss://example.com/v1/responses",
            "test-token",
            Some("org_123"),
        )
        .expect("request should build");

        assert_eq!(
            request
                .headers()
                .get("chatgpt-account-id")
                .and_then(|v| v.to_str().ok()),
            Some("org_123")
        );
    }

    #[test]
    fn builds_responses_ws_create_event_for_tools() {
        let request = CompletionRequest {
            messages: vec![
                Message {
                    role: Role::System,
                    content: vec![ContentPart::Text {
                        text: "System prompt".to_string(),
                    }],
                },
                Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: "Inspect the repo".to_string(),
                    }],
                },
            ],
            tools: vec![ToolDefinition {
                name: "read".to_string(),
                description: "Read a file".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" }
                    },
                    "required": ["path"]
                }),
            }],
            model: "gpt-5.4".to_string(),
            temperature: None,
            top_p: None,
            max_tokens: Some(8192),
            stop: Vec::new(),
        };

        let event = OpenAiCodexProvider::build_responses_ws_create_event(
            &request,
            "gpt-5.4",
            Some(ThinkingLevel::High),
            None,
        );

        assert_eq!(
            event.get("type").and_then(Value::as_str),
            Some("response.create")
        );
        assert_eq!(event.get("model").and_then(Value::as_str), Some("gpt-5.4"));
        assert_eq!(event.get("store").and_then(Value::as_bool), Some(false));
        assert_eq!(
            event.get("max_output_tokens").and_then(Value::as_u64),
            Some(8192)
        );
        assert_eq!(
            event.get("tools").and_then(Value::as_array).map(Vec::len),
            Some(1)
        );
    }

    #[tokio::test]
    async fn responses_ws_connection_round_trips_json_events() {
        let (client_io, server_io) = duplex(16 * 1024);

        let server = tokio::spawn(async move {
            let callback = |request: &ServerRequest, response: ServerResponse| {
                assert_eq!(request.uri().path(), "/v1/responses");
                assert_eq!(
                    request
                        .headers()
                        .get("Authorization")
                        .and_then(|v| v.to_str().ok()),
                    Some("Bearer test-token")
                );
                Ok(response)
            };

            let mut socket = accept_hdr_async(server_io, callback)
                .await
                .expect("server websocket handshake should succeed");
            let message = socket
                .next()
                .await
                .expect("server should receive message")
                .expect("server message should be valid");
            match message {
                WsMessage::Text(text) => {
                    let event: Value =
                        serde_json::from_str(&text).expect("server should parse JSON event");
                    assert_eq!(
                        event.get("type").and_then(Value::as_str),
                        Some("response.create")
                    );
                }
                other => panic!("expected text frame, got {other:?}"),
            }

            socket
                .send(WsMessage::Text(
                    json!({ "type": "response.created" }).to_string().into(),
                ))
                .await
                .expect("server should send response");
        });

        let request = OpenAiCodexProvider::build_responses_ws_request_with_base_url(
            "ws://localhost/v1/responses",
            "test-token",
        )
        .expect("client request should build");
        let (stream, _) = client_async(request, client_io)
            .await
            .expect("client websocket handshake should succeed");
        let mut connection = OpenAiRealtimeConnection::new(stream);

        connection
            .send_event(&json!({ "type": "response.create", "model": "gpt-5.4", "input": [] }))
            .await
            .expect("client should send event");
        let event = connection
            .recv_event()
            .await
            .expect("client should read event")
            .expect("client should receive session event");
        assert_eq!(
            event.get("type").and_then(Value::as_str),
            Some("response.created")
        );
        connection
            .close()
            .await
            .expect("client should close cleanly");

        server.await.expect("server task should finish");
    }

    #[test]
    fn responses_sse_parser_buffers_split_tool_call_events() {
        let mut parser = ResponsesSseParser::default();

        let chunk1 = br#"data: {"type":"response.output_item.added","item":{"type":"function_call","id":"fc_1","call_id":"call_1","name":"bash"}}

da"#;
        let first = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, chunk1);
        assert_eq!(first.len(), 1);
        match &first[0] {
            StreamChunk::ToolCallStart { id, name } => {
                assert_eq!(id, "call_1");
                assert_eq!(name, "bash");
            }
            other => panic!("expected tool start, got {other:?}"),
        }

        let chunk2 = br#"ta: {"type":"response.function_call_arguments.delta","item_id":"fc_1","delta":"{\"command\":"}

data: {"type":"response.function_call_arguments.delta","item_id":"fc_1","delta":"\"ls\"}"}

data: {"type":"response.output_item.done","item":{"type":"function_call","id":"fc_1","call_id":"call_1","name":"bash","arguments":"{\"command\":\"ls\"}"}}

data: {"type":"response.completed","response":{"usage":{"input_tokens":11,"output_tokens":7,"total_tokens":18}}}

"#;
        let second = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, chunk2);

        assert_eq!(second.len(), 4);
        match &second[0] {
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => {
                assert_eq!(id, "call_1");
                assert_eq!(arguments_delta, "{\"command\":");
            }
            other => panic!("expected first tool delta, got {other:?}"),
        }
        match &second[1] {
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => {
                assert_eq!(id, "call_1");
                assert_eq!(arguments_delta, "\"ls\"}");
            }
            other => panic!("expected second tool delta, got {other:?}"),
        }
        match &second[2] {
            StreamChunk::ToolCallEnd { id } => assert_eq!(id, "call_1"),
            other => panic!("expected tool end, got {other:?}"),
        }
        match &second[3] {
            StreamChunk::Done { usage } => {
                let usage = usage.as_ref().expect("expected usage");
                assert_eq!(usage.prompt_tokens, 11);
                assert_eq!(usage.completion_tokens, 7);
                assert_eq!(usage.total_tokens, 18);
            }
            other => panic!("expected done, got {other:?}"),
        }
    }

    #[test]
    fn responses_sse_parser_falls_back_to_done_item_arguments() {
        let mut parser = ResponsesSseParser::default();
        let bytes = br#"data: {"type":"response.output_item.done","item":{"type":"function_call","id":"fc_2","call_id":"call_2","name":"read","arguments":"{\"path\":\"src/main.rs\"}"}}

"#;

        let chunks = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, bytes);
        assert_eq!(chunks.len(), 3);
        match &chunks[0] {
            StreamChunk::ToolCallStart { id, name } => {
                assert_eq!(id, "call_2");
                assert_eq!(name, "read");
            }
            other => panic!("expected tool start, got {other:?}"),
        }
        match &chunks[1] {
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => {
                assert_eq!(id, "call_2");
                assert_eq!(arguments_delta, "{\"path\":\"src/main.rs\"}");
            }
            other => panic!("expected tool delta, got {other:?}"),
        }
        match &chunks[2] {
            StreamChunk::ToolCallEnd { id } => assert_eq!(id, "call_2"),
            other => panic!("expected tool end, got {other:?}"),
        }
    }

    #[test]
    fn responses_sse_parser_flushes_final_event_without_trailing_blank_line() {
        let mut parser = ResponsesSseParser::default();
        let bytes = br#"data: {"type":"response.output_item.done","item":{"type":"function_call","id":"fc_3","call_id":"call_3","name":"read","arguments":"{\"path\":\"src/lib.rs\"}"}}"#;

        let first = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, bytes);
        assert!(first.is_empty());

        let flushed = OpenAiCodexProvider::finish_responses_sse_parser(&mut parser);
        assert_eq!(flushed.len(), 3);
        match &flushed[0] {
            StreamChunk::ToolCallStart { id, name } => {
                assert_eq!(id, "call_3");
                assert_eq!(name, "read");
            }
            other => panic!("expected tool start, got {other:?}"),
        }
        match &flushed[1] {
            StreamChunk::ToolCallDelta {
                id,
                arguments_delta,
            } => {
                assert_eq!(id, "call_3");
                assert_eq!(arguments_delta, "{\"path\":\"src/lib.rs\"}");
            }
            other => panic!("expected tool delta, got {other:?}"),
        }
        match &flushed[2] {
            StreamChunk::ToolCallEnd { id } => assert_eq!(id, "call_3"),
            other => panic!("expected tool end, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn collect_stream_completion_updates_tool_name_after_early_delta() {
        let stream = stream::iter(vec![
            StreamChunk::ToolCallDelta {
                id: "call_4".to_string(),
                arguments_delta: "{\"path\":\"src/provider/openai_codex.rs\"}".to_string(),
            },
            StreamChunk::ToolCallStart {
                id: "call_4".to_string(),
                name: "read".to_string(),
            },
            StreamChunk::Done { usage: None },
        ]);

        let response = OpenAiCodexProvider::collect_stream_completion(Box::pin(stream))
            .await
            .expect("stream completion should succeed");

        assert!(matches!(
            response.message.content.first(),
            Some(ContentPart::ToolCall { id, name, arguments, .. })
                if id == "call_4"
                    && name == "read"
                    && arguments == "{\"path\":\"src/provider/openai_codex.rs\"}"
        ));
    }
}
