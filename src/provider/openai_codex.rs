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
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

const OPENAI_API_URL: &str = "https://api.openai.com/v1";
const AUTHORIZE_URL: &str = "https://auth.openai.com/oauth/authorize";
const TOKEN_URL: &str = "https://auth.openai.com/oauth/token";
const CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
const REDIRECT_URI: &str = "http://localhost:1455/auth/callback";
const SCOPE: &str = "openid profile email offline_access";

/// Cached OAuth tokens with expiration tracking
struct CachedTokens {
    access_token: String,
    refresh_token: String,
    expires_at: std::time::Instant,
}

/// Stored OAuth credentials (persisted to Vault)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCredentials {
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
    /// Stored credentials from Vault (for refresh on startup)
    stored_credentials: Option<OAuthCredentials>,
}

impl std::fmt::Debug for OpenAiCodexProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiCodexProvider")
            .field("has_credentials", &self.stored_credentials.is_some())
            .finish()
    }
}

impl OpenAiCodexProvider {
    /// Create from stored OAuth credentials (from Vault)
    pub fn from_credentials(credentials: OAuthCredentials) -> Self {
        Self {
            client: Client::new(),
            cached_tokens: Arc::new(RwLock::new(None)),
            stored_credentials: Some(credentials),
        }
    }

    /// Create a new unauthenticated instance (requires OAuth flow)
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cached_tokens: Arc::new(RwLock::new(None)),
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

            bytes[0..8].copy_from_slice(&ts_bytes);
            bytes[8..24].copy_from_slice(&tid_hash[0..16]);
            bytes[24..].copy_from_slice(&Sha256::digest(&ts_bytes)[0..8]);
            bytes
        };
        let verifier = URL_SAFE_NO_PAD.encode(&random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge_bytes = hasher.finalize();
        let challenge = URL_SAFE_NO_PAD.encode(&challenge_bytes);

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

    /// Exchange authorization code for tokens
    #[allow(dead_code)]
    pub async fn exchange_code(code: &str, verifier: &str) -> Result<OAuthCredentials> {
        let client = Client::new();
        let form_body = format!(
            "grant_type={}&client_id={}&code={}&code_verifier={}&redirect_uri={}",
            urlencoding::encode("authorization_code"),
            CLIENT_ID,
            urlencoding::encode(code),
            urlencoding::encode(verifier),
            urlencoding::encode(REDIRECT_URI),
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

        Ok(OAuthCredentials {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_at,
        })
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
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_at,
        })
    }

    /// Get a valid access token, refreshing if necessary
    async fn get_access_token(&self) -> Result<String> {
        {
            let cache = self.cached_tokens.read().await;
            if let Some(ref tokens) = *cache {
                if tokens
                    .expires_at
                    .duration_since(std::time::Instant::now())
                    .as_secs()
                    > 300
                {
                    return Ok(tokens.access_token.clone());
                }
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
                let new_creds = self.refresh_access_token(&stored.refresh_token).await?;
                new_creds
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

        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }

        let response = self
            .client
            .post(format!("{}/chat/completions", OPENAI_API_URL))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error ({}): {}", status, body);
        }

        #[derive(Deserialize)]
        struct OpenAiResponse {
            choices: Vec<OpenAiChoice>,
            usage: Option<OpenAiUsage>,
        }

        #[derive(Deserialize)]
        struct OpenAiChoice {
            message: OpenAiMessage,
            finish_reason: Option<String>,
        }

        #[derive(Deserialize)]
        struct OpenAiMessage {
            content: Option<String>,
            tool_calls: Option<Vec<OpenAiToolCall>>,
        }

        #[derive(Deserialize)]
        struct OpenAiToolCall {
            id: String,
            function: OpenAiFunction,
        }

        #[derive(Deserialize)]
        struct OpenAiFunction {
            name: String,
            arguments: String,
        }

        #[derive(Deserialize)]
        struct OpenAiUsage {
            prompt_tokens: usize,
            completion_tokens: usize,
            total_tokens: usize,
        }

        let openai_resp: OpenAiResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        let choice = openai_resp
            .choices
            .into_iter()
            .next()
            .context("No choices in response")?;

        let mut content = Vec::new();

        if let Some(text) = choice.message.content {
            if !text.is_empty() {
                content.push(ContentPart::Text { text });
            }
        }

        if let Some(tool_calls) = choice.message.tool_calls {
            for tc in tool_calls {
                content.push(ContentPart::ToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    arguments: tc.function.arguments,
                });
            }
        }

        let finish_reason = match choice.finish_reason.as_deref() {
            Some("stop") => FinishReason::Stop,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("length") => FinishReason::Length,
            _ => FinishReason::Stop,
        };

        let usage = openai_resp
            .usage
            .map(|u| Usage {
                prompt_tokens: u.prompt_tokens,
                completion_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
                cache_read_tokens: None,
                cache_write_tokens: None,
            })
            .unwrap_or_default();

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage,
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        let access_token = self.get_access_token().await?;

        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "stream": true,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }

        let response = self
            .client
            .post(format!("{}/chat/completions", OPENAI_API_URL))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to OpenAI")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error ({}): {}", status, body);
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

                    #[derive(Deserialize)]
                    struct StreamResponse {
                        choices: Vec<StreamChoice>,
                    }
                    #[derive(Deserialize)]
                    struct StreamChoice {
                        delta: StreamDelta,
                        #[allow(dead_code)]
                        finish_reason: Option<String>,
                    }
                    #[derive(Deserialize)]
                    struct StreamDelta {
                        content: Option<String>,
                        tool_calls: Option<Vec<StreamToolCall>>,
                    }
                    #[derive(Deserialize)]
                    struct StreamToolCall {
                        id: Option<String>,
                        function: Option<StreamFunction>,
                    }
                    #[derive(Deserialize)]
                    struct StreamFunction {
                        name: Option<String>,
                        arguments: Option<String>,
                    }

                    if let Ok(resp) = serde_json::from_str::<StreamResponse>(data) {
                        for choice in resp.choices {
                            if let Some(content) = choice.delta.content {
                                chunks.push(StreamChunk::Text(content));
                            }
                            if let Some(tool_calls) = choice.delta.tool_calls {
                                for tc in tool_calls {
                                    if let Some(id) = &tc.id {
                                        if let Some(func) = &tc.function {
                                            if let Some(name) = &func.name {
                                                chunks.push(StreamChunk::ToolCallStart {
                                                    id: id.clone(),
                                                    name: name.clone(),
                                                });
                                            }
                                            if let Some(args) = &func.arguments {
                                                chunks.push(StreamChunk::ToolCallDelta {
                                                    id: id.clone(),
                                                    arguments_delta: args.clone(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                futures::stream::iter(chunks)
            }
            Err(e) => futures::stream::iter(vec![StreamChunk::Error(e.to_string())]),
        });

        Ok(Box::pin(stream))
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
}
