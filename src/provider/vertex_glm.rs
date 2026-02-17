//! Vertex AI GLM provider implementation (MaaS endpoint)
//!
//! GLM-5 via Google Cloud Vertex AI Managed API Service.
//! Uses service account JWT auth to obtain OAuth2 access tokens.
//! The service account JSON key is stored in Vault and used to sign JWTs
//! that are exchanged for short-lived access tokens (cached ~55 min).
//!
//! Reference: https://console.cloud.google.com/vertex-ai/publishers/zai/model-garden/glm-5

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RETRIES: u32 = 3;

const VERTEX_ENDPOINT: &str = "aiplatform.googleapis.com";
const VERTEX_REGION: &str = "global";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const VERTEX_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";

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
#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    iat: u64,
    exp: u64,
}

pub struct VertexGlmProvider {
    client: Client,
    project_id: String,
    base_url: String,
    sa_key: ServiceAccountKey,
    encoding_key: EncodingKey,
    /// Cached OAuth2 access token (refreshes ~5 min before expiry)
    cached_token: Arc<RwLock<Option<CachedToken>>>,
}

impl std::fmt::Debug for VertexGlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexGlmProvider")
            .field("project_id", &self.project_id)
            .field("base_url", &self.base_url)
            .field("client_email", &self.sa_key.client_email)
            .finish()
    }
}

impl VertexGlmProvider {
    /// Create from a service account JSON key string
    pub fn new(sa_json: &str, project_id: Option<String>) -> Result<Self> {
        let sa_key: ServiceAccountKey =
            serde_json::from_str(sa_json).context("Failed to parse service account JSON key")?;

        let project_id = project_id
            .or_else(|| sa_key.project_id.clone())
            .ok_or_else(|| anyhow::anyhow!("No project_id found in SA key or Vault config"))?;

        let encoding_key = EncodingKey::from_rsa_pem(sa_key.private_key.as_bytes())
            .context("Failed to parse RSA private key from service account")?;

        let base_url = format!(
            "https://{}/v1/projects/{}/locations/{}/endpoints/openapi",
            VERTEX_ENDPOINT, project_id, VERTEX_REGION
        );

        tracing::debug!(
            provider = "vertex-glm",
            project_id = %project_id,
            client_email = %sa_key.client_email,
            base_url = %base_url,
            "Creating Vertex GLM provider with service account"
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
                            json!({"role": role, "content": ""})
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

                        let mut msg_json = json!({
                            "role": "assistant",
                            "content": if text.is_empty() { Value::Null } else { json!(text) },
                        });

                        if !tool_calls.is_empty() {
                            msg_json["tool_calls"] = json!(tool_calls);
                        }
                        msg_json
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

                        json!({"role": role, "content": text})
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

// Response types
#[derive(Debug, Deserialize)]
struct ChatCompletion {
    choices: Vec<Choice>,
    #[serde(default)]
    usage: Option<ApiUsage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChoiceMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChoiceMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Deserialize)]
struct ToolCall {
    id: String,
    function: FunctionCall,
}

#[derive(Debug, Deserialize)]
struct FunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct ApiUsage {
    #[serde(default)]
    prompt_tokens: usize,
    #[serde(default)]
    completion_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    error: ApiErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ApiErrorDetail {
    message: String,
    #[serde(default, rename = "type")]
    error_type: Option<String>,
}

// SSE streaming types
#[derive(Debug, Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<StreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct StreamToolCall {
    #[serde(default)]
    id: Option<String>,
    function: Option<StreamFunction>,
}

#[derive(Debug, Deserialize)]
struct StreamFunction {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}

#[async_trait]
impl Provider for VertexGlmProvider {
    fn name(&self) -> &str {
        "vertex-glm"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "zai-org/glm-5-maas".to_string(),
                name: "GLM-5 (Vertex AI MaaS)".to_string(),
                provider: "vertex-glm".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(1.0),
                output_cost_per_million: Some(3.2),
            },
            ModelInfo {
                id: "glm-5".to_string(),
                name: "GLM-5 (Vertex AI)".to_string(),
                provider: "vertex-glm".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(1.0),
                output_cost_per_million: Some(3.2),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let mut access_token = self.get_access_token().await?;

        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Resolve model ID to Vertex format
        let model = if request.model.starts_with("zai-org/") {
            request.model.clone()
        } else {
            format!(
                "zai-org/{}-maas",
                request.model.trim_start_matches("zai-org/")
            )
        };

        // GLM-5 defaults to temperature 1.0 for best results
        let temperature = request.temperature.unwrap_or(1.0);

        let mut body = json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "stream": false,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(model = %request.model, "Vertex GLM request");

        let url = format!("{}/chat/completions", self.base_url);
        let mut last_err = None;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let backoff = Duration::from_millis(1000 * 2u64.pow(attempt - 1));
                tracing::warn!(
                    attempt,
                    backoff_ms = backoff.as_millis() as u64,
                    "Vertex GLM retrying after transient error"
                );
                tokio::time::sleep(backoff).await;
                // Re-acquire token in case it expired during backoff
                access_token = self.get_access_token().await?;
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
                    tracing::warn!(error = %e, "Vertex GLM request timed out");
                    last_err = Some(format!("Request timed out: {e}"));
                    continue;
                }
                Err(e) => anyhow::bail!("Failed to send request to Vertex AI GLM: {e}"),
            };

            let status = response.status();
            let text = response
                .text()
                .await
                .context("Failed to read Vertex AI GLM response")?;

            if status == reqwest::StatusCode::SERVICE_UNAVAILABLE && attempt + 1 < MAX_RETRIES {
                tracing::warn!(status = %status, body = %text, "Vertex GLM service unavailable, retrying");
                last_err = Some(format!("503 Service Unavailable: {text}"));
                continue;
            }

            if !status.is_success() {
                if let Ok(err) = serde_json::from_str::<ApiError>(&text) {
                    anyhow::bail!(
                        "Vertex AI GLM API error: {} ({:?})",
                        err.error.message,
                        err.error.error_type
                    );
                }
                anyhow::bail!("Vertex AI GLM API error: {} {}", status, text);
            }

            let completion: ChatCompletion = serde_json::from_str(&text).context(format!(
                "Failed to parse Vertex AI GLM response: {}",
                &text[..text.len().min(200)]
            ))?;

            let choice = completion
                .choices
                .first()
                .ok_or_else(|| anyhow::anyhow!("No choices in Vertex AI GLM response"))?;

            let mut content = Vec::new();
            let mut has_tool_calls = false;

            if let Some(text) = &choice.message.content {
                if !text.is_empty() {
                    content.push(ContentPart::Text { text: text.clone() });
                }
            }

            if let Some(tool_calls) = &choice.message.tool_calls {
                has_tool_calls = !tool_calls.is_empty();
                for tc in tool_calls {
                    content.push(ContentPart::ToolCall {
                        id: tc.id.clone(),
                        name: tc.function.name.clone(),
                        arguments: tc.function.arguments.clone(),
                    });
                }
            }

            let finish_reason = if has_tool_calls {
                FinishReason::ToolCalls
            } else {
                match choice.finish_reason.as_deref() {
                    Some("stop") => FinishReason::Stop,
                    Some("length") => FinishReason::Length,
                    Some("tool_calls") => FinishReason::ToolCalls,
                    Some("content_filter") => FinishReason::ContentFilter,
                    _ => FinishReason::Stop,
                }
            };

            return Ok(CompletionResponse {
                message: Message {
                    role: Role::Assistant,
                    content,
                },
                usage: Usage {
                    prompt_tokens: completion
                        .usage
                        .as_ref()
                        .map(|u| u.prompt_tokens)
                        .unwrap_or(0),
                    completion_tokens: completion
                        .usage
                        .as_ref()
                        .map(|u| u.completion_tokens)
                        .unwrap_or(0),
                    total_tokens: completion
                        .usage
                        .as_ref()
                        .map(|u| u.total_tokens)
                        .unwrap_or(0),
                    cache_read_tokens: None,
                    cache_write_tokens: None,
                },
                finish_reason,
            });
        }

        anyhow::bail!(
            "Vertex AI GLM request failed after {MAX_RETRIES} attempts: {}",
            last_err.unwrap_or_default()
        )
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let mut access_token = self.get_access_token().await?;

        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Resolve model ID to Vertex format
        let model = if request.model.starts_with("zai-org") {
            request.model.clone()
        } else {
            format!(
                "zai-org/{}-maas",
                request.model.trim_start_matches("zai-org/")
            )
        };

        let temperature = request.temperature.unwrap_or(1.0);

        let mut body = json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "stream": true,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(model = %request.model, "Vertex GLM streaming request");

        let url = format!("{}/chat/completions", self.base_url);
        let mut last_err = String::new();

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let backoff = Duration::from_millis(1000 * 2u64.pow(attempt - 1));
                tracing::warn!(
                    attempt,
                    backoff_ms = backoff.as_millis() as u64,
                    "Vertex GLM streaming retrying after transient error"
                );
                tokio::time::sleep(backoff).await;
                access_token = self.get_access_token().await?;
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
                    tracing::warn!(error = %e, "Vertex GLM streaming request timed out");
                    last_err = format!("Request timed out: {e}");
                    continue;
                }
                Err(e) => anyhow::bail!("Failed to send streaming request to Vertex AI GLM: {e}"),
            };

            if response.status() == reqwest::StatusCode::SERVICE_UNAVAILABLE
                && attempt + 1 < MAX_RETRIES
            {
                let text = response.text().await.unwrap_or_default();
                tracing::warn!(body = %text, "Vertex GLM streaming service unavailable, retrying");
                last_err = format!("503 Service Unavailable: {text}");
                continue;
            }

            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                if let Ok(err) = serde_json::from_str::<ApiError>(&text) {
                    anyhow::bail!(
                        "Vertex AI GLM API error: {} ({:?})",
                        err.error.message,
                        err.error.error_type
                    );
                }
                anyhow::bail!("Vertex AI GLM streaming error: {} {}", status, text);
            }

            let stream = response.bytes_stream();
            let mut buffer = String::new();

            return Ok(stream
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

                                if line == "data: [DONE]" {
                                    if !text_buf.is_empty() {
                                        chunks
                                            .push(StreamChunk::Text(std::mem::take(&mut text_buf)));
                                    }
                                    chunks.push(StreamChunk::Done { usage: None });
                                    continue;
                                }
                                if let Some(data) = line.strip_prefix("data: ") {
                                    if let Ok(parsed) = serde_json::from_str::<StreamResponse>(data)
                                    {
                                        if let Some(choice) = parsed.choices.first() {
                                            if let Some(ref content) = choice.delta.content {
                                                text_buf.push_str(content);
                                            }
                                            if let Some(ref tool_calls) = choice.delta.tool_calls {
                                                if !text_buf.is_empty() {
                                                    chunks.push(StreamChunk::Text(std::mem::take(
                                                        &mut text_buf,
                                                    )));
                                                }
                                                for tc in tool_calls {
                                                    if let Some(ref func) = tc.function {
                                                        let id = tc.id.clone().unwrap_or_default();
                                                        if let Some(ref name) = func.name {
                                                            chunks.push(
                                                                StreamChunk::ToolCallStart {
                                                                    id: id.clone(),
                                                                    name: name.clone(),
                                                                },
                                                            );
                                                        }
                                                        if let Some(ref args) = func.arguments {
                                                            chunks.push(
                                                                StreamChunk::ToolCallDelta {
                                                                    id: id.clone(),
                                                                    arguments_delta: args.clone(),
                                                                },
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                            if let Some(ref reason) = choice.finish_reason {
                                                if !text_buf.is_empty() {
                                                    chunks.push(StreamChunk::Text(std::mem::take(
                                                        &mut text_buf,
                                                    )));
                                                }
                                                if reason == "tool_calls" {
                                                    if let Some(tc) = choice
                                                        .delta
                                                        .tool_calls
                                                        .as_ref()
                                                        .and_then(|t| t.last())
                                                    {
                                                        if let Some(id) = &tc.id {
                                                            chunks.push(StreamChunk::ToolCallEnd {
                                                                id: id.clone(),
                                                            });
                                                        }
                                                    }
                                                }
                                            }
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
                .boxed());
        }

        anyhow::bail!("Vertex AI GLM streaming failed after {MAX_RETRIES} attempts: {last_err}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rejects_invalid_sa_json() {
        let result = VertexGlmProvider::new("{}", None);
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
        // Invalid RSA key but the error should be about key parsing, not project
        let result = VertexGlmProvider::new(&sa_json.to_string(), None);
        assert!(result.is_err());
    }
}
