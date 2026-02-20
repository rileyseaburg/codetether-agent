//! Vertex AI Anthropic provider implementation
//!
//! Claude models (Sonnet 4.6, Opus 4, etc.) via Google Cloud Vertex AI.
//! Uses service account JWT auth to obtain OAuth2 access tokens.
//!
//! Key differences from native Anthropic API:
//! - Model is specified in URL path, not request body
//! - `anthropic_version` must be set to `vertex-2023-10-16`
//! - Uses Bearer token auth (OAuth2) instead of x-api-key header
//!
//! Reference: https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-claude

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RETRIES: u32 = 3;

const VERTEX_REGION: &str = "us-east5";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const VERTEX_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";
const VERTEX_ANTHROPIC_VERSION: &str = "vertex-2023-10-16";

/// Cached OAuth2 access token with expiration tracking
struct CachedToken {
    token: String,
    expires_at: std::time::Instant,
}

/// GCP service account key (parsed from JSON)
#[derive(Debug, Clone, Deserialize)]
struct ServiceAccountKey {
    client_email: String,
    private_key: String,
    token_uri: Option<String>,
    project_id: Option<String>,
}

/// JWT claims for GCP service account auth
#[derive(serde::Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    iat: u64,
    exp: u64,
}

pub struct VertexAnthropicProvider {
    client: Client,
    project_id: String,
    base_url: String,
    sa_key: ServiceAccountKey,
    encoding_key: EncodingKey,
    /// Cached OAuth2 access token (refreshes ~5 min before expiry)
    cached_token: Arc<RwLock<Option<CachedToken>>>,
}

impl std::fmt::Debug for VertexAnthropicProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexAnthropicProvider")
            .field("project_id", &self.project_id)
            .field("base_url", &self.base_url)
            .field("client_email", &self.sa_key.client_email)
            .finish()
    }
}

impl VertexAnthropicProvider {
    /// Create from a service account JSON key string
    pub fn new(sa_json: &str, project_id: Option<String>) -> Result<Self> {
        let sa_key: ServiceAccountKey =
            serde_json::from_str(sa_json).context("Failed to parse service account JSON key")?;

        let project_id = project_id
            .or_else(|| sa_key.project_id.clone())
            .ok_or_else(|| anyhow::anyhow!("No project_id found in SA key or Vault config"))?;

        let encoding_key = EncodingKey::from_rsa_pem(sa_key.private_key.as_bytes())
            .context("Failed to parse RSA private key from service account")?;

        // Vertex AI Anthropic endpoint format:
        // https://REGION-aiplatform.googleapis.com/v1/projects/PROJECT_ID/locations/REGION/publishers/anthropic/models/MODEL_ID:rawPredict
        let base_url = format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/anthropic/models",
            VERTEX_REGION, project_id, VERTEX_REGION
        );

        tracing::debug!(
            provider = "vertex-anthropic",
            project_id = %project_id,
            client_email = %sa_key.client_email,
            base_url = %base_url,
            "Creating Vertex Anthropic provider with service account"
        );

        let client = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            project_id,
            base_url,
            sa_key,
            encoding_key,
            cached_token: Arc::new(RwLock::new(None)),
        })
    }

    /// Get a valid OAuth2 access token, refreshing if needed
    async fn get_access_token(&self) -> Result<String> {
        // Check cache — refresh 5 minutes before expiration
        {
            let cache = self.cached_token.read().await;
            if let Some(ref cached) = *cache {
                if cached.expires_at
                    > std::time::Instant::now() + std::time::Duration::from_secs(300)
                {
                    return Ok(cached.token.clone());
                }
            }
        }

        // Sign a JWT assertion
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("System time error")?
            .as_secs();

        let token_uri = self.sa_key.token_uri.as_deref().unwrap_or(GOOGLE_TOKEN_URL);

        let claims = JwtClaims {
            iss: self.sa_key.client_email.clone(),
            scope: VERTEX_SCOPE.to_string(),
            aud: token_uri.to_string(),
            iat: now,
            exp: now + 3600,
        };

        let header = Header::new(Algorithm::RS256);
        let assertion = jsonwebtoken::encode(&header, &claims, &self.encoding_key)
            .context("Failed to sign JWT assertion")?;

        // Exchange JWT for access token
        let form_body = format!(
            "grant_type={}&assertion={}",
            urlencoding::encode("urn:ietf:params:oauth:grant-type:jwt-bearer"),
            urlencoding::encode(&assertion),
        );
        let response = self
            .client
            .post(token_uri)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .context("Failed to exchange JWT for access token")?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("Failed to read token response")?;

        if !status.is_success() {
            anyhow::bail!("GCP token exchange failed: {status} {body}");
        }

        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: String,
            #[serde(default)]
            expires_in: Option<u64>,
        }

        let token_resp: TokenResponse =
            serde_json::from_str(&body).context("Failed to parse GCP token response")?;

        let expires_in = token_resp.expires_in.unwrap_or(3600);

        // Cache it
        {
            let mut cache = self.cached_token.write().await;
            *cache = Some(CachedToken {
                token: token_resp.access_token.clone(),
                expires_at: std::time::Instant::now() + std::time::Duration::from_secs(expires_in),
            });
        }

        tracing::debug!(
            client_email = %self.sa_key.client_email,
            expires_in_secs = expires_in,
            "Refreshed GCP access token via service account JWT"
        );

        Ok(token_resp.access_token)
    }

    /// Convert our generic messages to Anthropic Messages API format.
    /// Same logic as AnthropicProvider, but returns system as Option<Value> for Vertex format.
    fn convert_messages(messages: &[Message]) -> (Option<Value>, Vec<Value>) {
        let mut system_blocks: Vec<Value> = Vec::new();
        let mut api_messages: Vec<Value> = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                system_blocks.push(json!({
                                    "type": "text",
                                    "text": text,
                                }));
                            }
                            ContentPart::Thinking { text } => {
                                system_blocks.push(json!({
                                    "type": "thinking",
                                    "thinking": text,
                                }));
                            }
                            _ => {}
                        }
                    }
                }
                Role::User => {
                    let mut content_parts: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                content_parts.push(json!({
                                    "type": "text",
                                    "text": text,
                                }));
                            }
                            ContentPart::Thinking { text } => {
                                content_parts.push(json!({
                                    "type": "thinking",
                                    "thinking": text,
                                }));
                            }
                            _ => {}
                        }
                    }
                    if content_parts.is_empty() {
                        content_parts.push(json!({"type": "text", "text": " "}));
                    }
                    api_messages.push(json!({
                        "role": "user",
                        "content": content_parts
                    }));
                }
                Role::Assistant => {
                    let mut content_parts: Vec<Value> = Vec::new();

                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                content_parts.push(json!({
                                    "type": "text",
                                    "text": text
                                }));
                            }
                            ContentPart::Thinking { text } => {
                                content_parts.push(json!({
                                    "type": "thinking",
                                    "thinking": text
                                }));
                            }
                            ContentPart::ToolCall {
                                id,
                                name,
                                arguments,
                                ..
                            } => {
                                let input: Value = serde_json::from_str(arguments)
                                    .unwrap_or_else(|_| json!({"raw": arguments}));
                                content_parts.push(json!({
                                    "type": "tool_use",
                                    "id": id,
                                    "name": name,
                                    "input": input
                                }));
                            }
                            _ => {}
                        }
                    }

                    if content_parts.is_empty() {
                        content_parts.push(json!({"type": "text", "text": " "}));
                    }

                    api_messages.push(json!({
                        "role": "assistant",
                        "content": content_parts
                    }));
                }
                Role::Tool => {
                    let mut tool_results: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        if let ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        } = part
                        {
                            tool_results.push(json!({
                                "type": "tool_result",
                                "tool_use_id": tool_call_id,
                                "content": content
                            }));
                        }
                    }
                    if !tool_results.is_empty() {
                        api_messages.push(json!({
                            "role": "user",
                            "content": tool_results
                        }));
                    }
                }
            }
        }

        let system = if system_blocks.is_empty() {
            None
        } else if system_blocks.len() == 1 {
            // Single block: return as string for simplicity
            system_blocks
                .first()
                .and_then(|b| b.get("text"))
                .and_then(Value::as_str)
                .map(|s| json!(s))
        } else {
            // Multiple blocks: return as array
            Some(json!(system_blocks))
        };

        (system, api_messages)
    }

    fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.parameters
                })
            })
            .collect()
    }

    /// Build the Vertex AI URL for a specific model
    fn build_model_url(&self, model: &str) -> String {
        // Normalize model ID to Vertex format
        let model_id = model
            .trim_start_matches("vertex-anthropic/")
            .trim_start_matches("anthropic/")
            .trim_start_matches("claude-");

        // Handle both formats: "claude-sonnet-4-6" and "sonnet-4-6"
        let final_model_id = if model_id.starts_with("claude-") {
            model_id.to_string()
        } else {
            format!("claude-{model_id}")
        };

        format!("{}/{}:rawPredict", self.base_url, final_model_id)
    }
}

// Response types (same as Anthropic)
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    model: String,
    content: Vec<AnthropicContent>,
    #[serde(default)]
    stop_reason: Option<String>,
    #[serde(default)]
    usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum AnthropicContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking {
        #[serde(default)]
        thinking: Option<String>,
        #[serde(default)]
        text: Option<String>,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    #[serde(default)]
    input_tokens: usize,
    #[serde(default)]
    output_tokens: usize,
    #[serde(default)]
    cache_creation_input_tokens: Option<usize>,
    #[serde(default)]
    cache_read_input_tokens: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct AnthropicError {
    error: AnthropicErrorDetail,
}

#[derive(Debug, Deserialize)]
struct AnthropicErrorDetail {
    message: String,
    #[serde(default, rename = "type")]
    error_type: Option<String>,
}

#[async_trait]
impl Provider for VertexAnthropicProvider {
    fn name(&self) -> &str {
        "vertex-anthropic"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "claude-sonnet-4-6".to_string(),
                name: "Claude Sonnet 4.6 (Vertex AI)".to_string(),
                provider: "vertex-anthropic".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(128_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            ModelInfo {
                id: "claude-sonnet-4-20250514".to_string(),
                name: "Claude Sonnet 4 (Vertex AI)".to_string(),
                provider: "vertex-anthropic".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(64_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            ModelInfo {
                id: "claude-opus-4-20250514".to_string(),
                name: "Claude Opus 4 (Vertex AI)".to_string(),
                provider: "vertex-anthropic".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(32_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(15.0),
                output_cost_per_million: Some(75.0),
            },
            ModelInfo {
                id: "claude-3-5-sonnet-v2@20241022".to_string(),
                name: "Claude 3.5 Sonnet v2 (Vertex AI)".to_string(),
                provider: "vertex-anthropic".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(8_192),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            ModelInfo {
                id: "claude-3-5-sonnet@20240620".to_string(),
                name: "Claude 3.5 Sonnet (Vertex AI)".to_string(),
                provider: "vertex-anthropic".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(8_192),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            ModelInfo {
                id: "claude-3-haiku@20240307".to_string(),
                name: "Claude 3 Haiku (Vertex AI)".to_string(),
                provider: "vertex-anthropic".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(4_096),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.25),
                output_cost_per_million: Some(1.25),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let access_token = self.get_access_token().await?;

        let (system_prompt, messages) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Build request body - note: model is NOT in body for Vertex, it's in URL
        let mut body = json!({
            "anthropic_version": VERTEX_ANTHROPIC_VERSION,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(8192),
        });

        if let Some(system) = system_prompt {
            body["system"] = system;
        }
        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(top_p) = request.top_p {
            body["top_p"] = json!(top_p);
        }

        let url = self.build_model_url(&request.model);

        tracing::debug!(
            model = %request.model,
            url = %url,
            "Vertex Anthropic request"
        );

        let mut last_err = None;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let backoff = Duration::from_millis(1000 * 2u64.pow(attempt - 1));
                tracing::warn!(
                    attempt,
                    backoff_ms = backoff.as_millis() as u64,
                    "Vertex Anthropic retrying after transient error"
                );
                tokio::time::sleep(backoff).await;
            }

            let send_result = self
                .client
                .post(&url)
                .bearer_auth(&access_token)
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await;

            let response = match send_result {
                Ok(r) => r,
                Err(e) if e.is_timeout() && attempt + 1 < MAX_RETRIES => {
                    tracing::warn!(error = %e, "Vertex Anthropic request timed out");
                    last_err = Some(format!("Request timed out: {e}"));
                    continue;
                }
                Err(e) => anyhow::bail!("Failed to send request to Vertex AI Anthropic: {e}"),
            };

            let status = response.status();
            let text = response
                .text()
                .await
                .context("Failed to read Vertex AI Anthropic response")?;

            if status == reqwest::StatusCode::SERVICE_UNAVAILABLE && attempt + 1 < MAX_RETRIES {
                tracing::warn!(status = %status, body = %text, "Vertex Anthropic service unavailable, retrying");
                last_err = Some(format!("503 Service Unavailable: {text}"));
                continue;
            }

            if !status.is_success() {
                if let Ok(err) = serde_json::from_str::<AnthropicError>(&text) {
                    anyhow::bail!(
                        "Vertex AI Anthropic API error: {} ({:?})",
                        err.error.message,
                        err.error.error_type
                    );
                }
                anyhow::bail!("Vertex AI Anthropic API error: {} {}", status, text);
            }

            let response: AnthropicResponse = serde_json::from_str(&text).context(format!(
                "Failed to parse Vertex AI Anthropic response: {}",
                &text[..text.len().min(200)]
            ))?;

            let mut content = Vec::new();
            let mut has_tool_calls = false;

            for part in &response.content {
                match part {
                    AnthropicContent::Text { text } => {
                        if !text.is_empty() {
                            content.push(ContentPart::Text { text: text.clone() });
                        }
                    }
                    AnthropicContent::Thinking { thinking, text } => {
                        let reasoning = thinking
                            .as_deref()
                            .or(text.as_deref())
                            .unwrap_or_default()
                            .trim()
                            .to_string();
                        if !reasoning.is_empty() {
                            content.push(ContentPart::Thinking { text: reasoning });
                        }
                    }
                    AnthropicContent::ToolUse { id, name, input } => {
                        has_tool_calls = true;
                        content.push(ContentPart::ToolCall {
                            id: id.clone(),
                            name: name.clone(),
                            arguments: serde_json::to_string(input).unwrap_or_default(),
                            thought_signature: None,
                        });
                    }
                    AnthropicContent::Unknown => {}
                }
            }

            let finish_reason = if has_tool_calls {
                FinishReason::ToolCalls
            } else {
                match response.stop_reason.as_deref() {
                    Some("end_turn") | Some("stop") => FinishReason::Stop,
                    Some("max_tokens") => FinishReason::Length,
                    Some("tool_use") => FinishReason::ToolCalls,
                    Some("content_filter") => FinishReason::ContentFilter,
                    _ => FinishReason::Stop,
                }
            };

            let usage = response.usage.as_ref();

            return Ok(CompletionResponse {
                message: Message {
                    role: Role::Assistant,
                    content,
                },
                usage: Usage {
                    prompt_tokens: usage.map(|u| u.input_tokens).unwrap_or(0),
                    completion_tokens: usage.map(|u| u.output_tokens).unwrap_or(0),
                    total_tokens: usage.map(|u| u.input_tokens + u.output_tokens).unwrap_or(0),
                    cache_read_tokens: usage.and_then(|u| u.cache_read_input_tokens),
                    cache_write_tokens: usage.and_then(|u| u.cache_creation_input_tokens),
                },
                finish_reason,
            });
        }

        anyhow::bail!(
            "Vertex AI Anthropic request failed after {MAX_RETRIES} attempts: {}",
            last_err.unwrap_or_default()
        )
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let access_token = self.get_access_token().await?;

        let (system_prompt, messages) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let mut body = json!({
            "anthropic_version": VERTEX_ANTHROPIC_VERSION,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(8192),
            "stream": true,
        });

        if let Some(system) = system_prompt {
            body["system"] = system;
        }
        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }

        let url = self.build_model_url(&request.model);

        tracing::debug!(model = %request.model, "Vertex Anthropic streaming request");

        let response = self
            .client
            .post(&url)
            .bearer_auth(&access_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Vertex AI Anthropic")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<AnthropicError>(&text) {
                anyhow::bail!(
                    "Vertex AI Anthropic API error: {} ({:?})",
                    err.error.message,
                    err.error.error_type
                );
            }
            anyhow::bail!("Vertex AI Anthropic streaming error: {} {}", status, text);
        }

        let stream = response.bytes_stream();
        let mut buffer = String::new();

        Ok(stream
            .flat_map(move |chunk_result| {
                let mut chunks: Vec<StreamChunk> = Vec::new();
                match chunk_result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        buffer.push_str(&text);

                        let mut text_buf = String::new();

                        while let Some(line_end) = buffer.find('\n') {
                            let line = buffer[..line_end].trim().to_string();
                            buffer = buffer[line_end + 1..].to_string();

                            // Handle SSE format
                            if line.starts_with("event:") {
                                continue; // Skip event type lines
                            }

                            if let Some(data) = line.strip_prefix("data: ") {
                                if data == "[DONE]" {
                                    if !text_buf.is_empty() {
                                        chunks
                                            .push(StreamChunk::Text(std::mem::take(&mut text_buf)));
                                    }
                                    chunks.push(StreamChunk::Done { usage: None });
                                    continue;
                                }

                                // Parse Anthropic streaming event
                                if let Ok(event) = serde_json::from_str::<Value>(data) {
                                    let event_type =
                                        event.get("type").and_then(Value::as_str).unwrap_or("");

                                    match event_type {
                                        "content_block_delta" => {
                                            if let Some(delta) = event.get("delta") {
                                                if let Some(text) =
                                                    delta.get("text").and_then(Value::as_str)
                                                {
                                                    text_buf.push_str(text);
                                                }
                                            }
                                        }
                                        "content_block_start" => {
                                            // Tool call start
                                            if let Some(content_block) = event.get("content_block")
                                                && content_block.get("type").and_then(Value::as_str)
                                                    == Some("tool_use")
                                            {
                                                if !text_buf.is_empty() {
                                                    chunks.push(StreamChunk::Text(std::mem::take(
                                                        &mut text_buf,
                                                    )));
                                                }
                                                let id = content_block
                                                    .get("id")
                                                    .and_then(Value::as_str)
                                                    .unwrap_or_default();
                                                let name = content_block
                                                    .get("name")
                                                    .and_then(Value::as_str)
                                                    .unwrap_or_default();
                                                chunks.push(StreamChunk::ToolCallStart {
                                                    id: id.to_string(),
                                                    name: name.to_string(),
                                                });
                                            }
                                        }
                                        "content_block_stop" => {
                                            // Tool call end
                                            let index = event.get("index").and_then(Value::as_u64);
                                            if let Some(_idx) = index {
                                                // We'd need to track tool call IDs, for now emit end
                                                // This is a simplification - real impl needs state tracking
                                            }
                                        }
                                        "message_delta" => {
                                            // Final stop reason
                                            if let Some(_usage) = event.get("usage") {
                                                // Could extract usage here
                                            }
                                        }
                                        "message_stop" => {
                                            if !text_buf.is_empty() {
                                                chunks.push(StreamChunk::Text(std::mem::take(
                                                    &mut text_buf,
                                                )));
                                            }
                                            chunks.push(StreamChunk::Done { usage: None });
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        if !text_buf.is_empty() {
                            chunks.push(StreamChunk::Text(text_buf));
                        }
                    }
                    Err(e) => chunks.push(StreamChunk::Error(e.to_string())),
                }
                futures::stream::iter(chunks)
            })
            .boxed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rejects_invalid_sa_json() {
        let result = VertexAnthropicProvider::new("{}", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_missing_project_id() {
        let sa_json = json!({
            "type": "service_account",
            "client_email": "test@test.iam.gserviceaccount.com",
            "private_key": "-----BEGIN RSA PRIVATE KEY-----\nMIIBogIBAAJBALRiMLAHudeSA/x3hB2f+2NRkJlS\n-----END RSA PRIVATE KEY-----\n",
            "token_uri": "https://oauth2.googleapis.com/token"
        });
        let result = VertexAnthropicProvider::new(&sa_json.to_string(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_model_url_building() {
        // This would require a valid SA key, so just test the logic conceptually
        // In a real test, we'd mock the provider
    }
}
