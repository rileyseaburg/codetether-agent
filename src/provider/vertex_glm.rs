//! Vertex AI GLM provider implementation (MaaS endpoint)
//!
//! GLM-5 via Google Cloud Vertex AI Managed API Service.
//! Uses OpenAI-compatible chat completions endpoint with GCP Bearer token auth.
//! Reference: https://console.cloud.google.com/vertex-ai/publishers/zai/model-garden/glm-5

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::RwLock;

const VERTEX_ENDPOINT: &str = "aiplatform.googleapis.com";
const VERTEX_REGION: &str = "global";

/// Cached GCP access token with expiration tracking
struct CachedToken {
    token: String,
    expires_at: std::time::Instant,
}

pub struct VertexGlmProvider {
    client: Client,
    project_id: String,
    base_url: String,
    /// Cached access token (refreshes when expired)
    cached_token: Arc<RwLock<Option<CachedToken>>>,
    /// Service account key path (optional, for ADC fallback)
    service_account_key: Option<String>,
}

impl std::fmt::Debug for VertexGlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexGlmProvider")
            .field("project_id", &self.project_id)
            .field("base_url", &self.base_url)
            .field(
                "service_account_key",
                &self.service_account_key.as_ref().map(|_| "<SET>"),
            )
            .finish()
    }
}

impl VertexGlmProvider {
    /// Create a new Vertex GLM provider with project ID
    pub fn new(project_id: String) -> Result<Self> {
        Self::with_options(project_id, None, None)
    }

    /// Create with optional service account key path
    pub fn with_service_account(project_id: String, key_path: String) -> Result<Self> {
        Self::with_options(project_id, Some(key_path), None)
    }

    /// Create with full options
    pub fn with_options(
        project_id: String,
        service_account_key: Option<String>,
        base_url: Option<String>,
    ) -> Result<Self> {
        let base_url = base_url.unwrap_or_else(|| {
            format!(
                "https://{}/v1/projects/{}/locations/{}/endpoints/openapi",
                VERTEX_ENDPOINT, project_id, VERTEX_REGION
            )
        });

        tracing::debug!(
            provider = "vertex-glm",
            project_id = %project_id,
            base_url = %base_url,
            "Creating Vertex GLM provider"
        );

        Ok(Self {
            client: Client::new(),
            project_id,
            base_url,
            cached_token: Arc::new(RwLock::new(None)),
            service_account_key,
        })
    }

    /// Get a valid GCP access token, using cached token if still valid
    async fn get_access_token(&self) -> Result<String> {
        // Check cache first
        {
            let cache = self.cached_token.read().await;
            if let Some(ref cached) = *cache {
                // Refresh 5 minutes before expiration
                if cached.expires_at
                    > std::time::Instant::now() + std::time::Duration::from_secs(300)
                {
                    tracing::trace!("Using cached GCP access token");
                    return Ok(cached.token.clone());
                }
            }
        }

        // Generate new token
        let token = self.fetch_new_token()?;

        // Cache it
        {
            let mut cache = self.cached_token.write().await;
            *cache = Some(CachedToken {
                token: token.clone(),
                // GCP tokens typically expire in 1 hour
                expires_at: std::time::Instant::now() + std::time::Duration::from_secs(3600),
            });
        }

        tracing::debug!("Refreshed GCP access token");
        Ok(token)
    }

    /// Fetch a new access token using gcloud CLI or service account
    fn fetch_new_token(&self) -> Result<String> {
        // If service account key is set, use it
        if let Some(ref key_path) = self.service_account_key {
            return self.fetch_token_from_service_account(key_path);
        }

        // Otherwise use gcloud CLI with Application Default Credentials
        self.fetch_token_from_gcloud()
    }

    fn fetch_token_from_gcloud(&self) -> Result<String> {
        let output = Command::new("gcloud")
            .args(["auth", "print-access-token"])
            .output()
            .context(
                "Failed to execute gcloud CLI. Ensure gcloud is installed and authenticated.",
            )?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("gcloud auth failed: {}", stderr);
        }

        let token = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in gcloud output")?
            .trim()
            .to_string();

        if token.is_empty() {
            anyhow::bail!("gcloud returned empty access token");
        }

        Ok(token)
    }

    fn fetch_token_from_service_account(&self, key_path: &str) -> Result<String> {
        // Use gcloud with service account key
        let output = Command::new("gcloud")
            .args([
                "auth",
                "print-access-token",
                "--credential-file-override",
                key_path,
            ])
            .output()
            .context("Failed to execute gcloud with service account key")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("gcloud auth with service account failed: {}", stderr);
        }

        let token = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in gcloud output")?
            .trim()
            .to_string();

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
        let access_token = self.get_access_token().await?;

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

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&access_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Vertex AI GLM")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Vertex AI GLM response")?;

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

        Ok(CompletionResponse {
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
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let access_token = self.get_access_token().await?;

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

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&access_token)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Vertex AI GLM")?;

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

        Ok(stream
            .flat_map(move |chunk_result| {
                let mut chunks: Vec<StreamChunk> = Vec::new();
                match chunk_result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        buffer.push_str(&text);

                        let mut text_buf = String::new();
                        let mut pending_tool_calls: std::collections::HashMap<
                            String,
                            (String, String),
                        > = std::collections::HashMap::new();

                        while let Some(line_end) = buffer.find('\n') {
                            let line = buffer[..line_end].trim().to_string();
                            buffer = buffer[line_end + 1..].to_string();

                            if line == "data: [DONE]" {
                                if !text_buf.is_empty() {
                                    chunks.push(StreamChunk::Text(std::mem::take(&mut text_buf)));
                                }
                                chunks.push(StreamChunk::Done { usage: None });
                                continue;
                            }
                            if let Some(data) = line.strip_prefix("data: ") {
                                if let Ok(parsed) = serde_json::from_str::<StreamResponse>(data) {
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
                                                        chunks.push(StreamChunk::ToolCallStart {
                                                            id: id.clone(),
                                                            name: name.clone(),
                                                        });
                                                        pending_tool_calls.insert(
                                                            id.clone(),
                                                            (name.clone(), String::new()),
                                                        );
                                                    }
                                                    if let Some(ref args) = func.arguments {
                                                        chunks.push(StreamChunk::ToolCallDelta {
                                                            id: id.clone(),
                                                            arguments_delta: args.clone(),
                                                        });
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
            .boxed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_resolution() {
        // Model resolution happens in the complete() method
        // This test just verifies the provider can be created
        let provider = VertexGlmProvider::new("test-project".to_string());
        assert!(provider.is_ok());
    }
}
