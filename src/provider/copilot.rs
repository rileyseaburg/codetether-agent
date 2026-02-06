//! GitHub Copilot provider implementation using raw HTTP.

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};

const DEFAULT_BASE_URL: &str = "https://api.githubcopilot.com";
const COPILOT_PROVIDER: &str = "github-copilot";
const COPILOT_ENTERPRISE_PROVIDER: &str = "github-copilot-enterprise";

pub struct CopilotProvider {
    client: Client,
    token: String,
    base_url: String,
    provider_name: String,
}

impl CopilotProvider {
    pub fn new(token: String) -> Result<Self> {
        Self::with_base_url(token, DEFAULT_BASE_URL.to_string(), COPILOT_PROVIDER)
    }

    pub fn enterprise(token: String, enterprise_url: String) -> Result<Self> {
        let base_url = enterprise_base_url(&enterprise_url);
        Self::with_base_url(token, base_url, COPILOT_ENTERPRISE_PROVIDER)
    }

    pub fn with_base_url(token: String, base_url: String, provider_name: &str) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            token,
            base_url: base_url.trim_end_matches('/').to_string(),
            provider_name: provider_name.to_string(),
        })
    }

    fn user_agent() -> String {
        format!("codetether-agent/{}", env!("CARGO_PKG_VERSION"))
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
                                "content": if text.is_empty() { "".to_string() } else { text },
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

    fn is_agent_initiated(messages: &[Message]) -> bool {
        messages
            .iter()
            .rev()
            .find(|msg| msg.role != Role::System)
            .map(|msg| msg.role != Role::User)
            .unwrap_or(false)
    }

    fn has_vision_input(messages: &[Message]) -> bool {
        messages.iter().any(|msg| {
            msg.content
                .iter()
                .any(|part| matches!(part, ContentPart::Image { .. }))
        })
    }
}

#[derive(Debug, Deserialize)]
struct CopilotResponse {
    choices: Vec<CopilotChoice>,
    #[serde(default)]
    usage: Option<CopilotUsage>,
}

#[derive(Debug, Deserialize)]
struct CopilotChoice {
    message: CopilotMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CopilotMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<CopilotToolCall>>,
}

#[derive(Debug, Deserialize)]
struct CopilotToolCall {
    id: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    call_type: String,
    function: CopilotFunction,
}

#[derive(Debug, Deserialize)]
struct CopilotFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct CopilotUsage {
    #[serde(default)]
    prompt_tokens: usize,
    #[serde(default)]
    completion_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct CopilotErrorResponse {
    error: Option<CopilotErrorDetail>,
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CopilotErrorDetail {
    message: Option<String>,
    code: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CopilotModelsResponse {
    data: Vec<CopilotModelInfo>,
}

#[derive(Debug, Deserialize)]
struct CopilotModelInfo {
    id: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    model_picker_enabled: Option<bool>,
    #[serde(default)]
    policy: Option<CopilotModelPolicy>,
    #[serde(default)]
    capabilities: Option<CopilotModelCapabilities>,
}

#[derive(Debug, Deserialize)]
struct CopilotModelPolicy {
    #[serde(default)]
    state: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CopilotModelCapabilities {
    #[serde(default)]
    limits: Option<CopilotModelLimits>,
    #[serde(default)]
    supports: Option<CopilotModelSupports>,
}

#[derive(Debug, Deserialize)]
struct CopilotModelLimits {
    #[serde(default)]
    max_context_window_tokens: Option<usize>,
    #[serde(default)]
    max_output_tokens: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct CopilotModelSupports {
    #[serde(default)]
    tool_calls: Option<bool>,
    #[serde(default)]
    vision: Option<bool>,
    #[serde(default)]
    streaming: Option<bool>,
}

#[async_trait]
impl Provider for CopilotProvider {
    fn name(&self) -> &str {
        &self.provider_name
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let response = self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Openai-Intent", "conversation-edits")
            .header("User-Agent", Self::user_agent())
            .send()
            .await
            .context("Failed to fetch Copilot models")?;

        let mut models: Vec<ModelInfo> = if response.status().is_success() {
            let parsed: CopilotModelsResponse = response
                .json()
                .await
                .unwrap_or(CopilotModelsResponse { data: vec![] });

            parsed
                .data
                .into_iter()
                .map(|model| {
                    let caps = model.capabilities.as_ref();
                    let limits = caps.and_then(|c| c.limits.as_ref());
                    let supports = caps.and_then(|c| c.supports.as_ref());

                    ModelInfo {
                        id: model.id.clone(),
                        name: model.name.unwrap_or_else(|| model.id.clone()),
                        provider: self.provider_name.clone(),
                        context_window: limits
                            .and_then(|l| l.max_context_window_tokens)
                            .unwrap_or(128_000),
                        max_output_tokens: limits
                            .and_then(|l| l.max_output_tokens)
                            .or(Some(16_384)),
                        supports_vision: supports.and_then(|s| s.vision).unwrap_or(false),
                        supports_tools: supports.and_then(|s| s.tool_calls).unwrap_or(true),
                        supports_streaming: supports.and_then(|s| s.streaming).unwrap_or(true),
                        input_cost_per_million: Some(0.0),
                        output_cost_per_million: Some(0.0),
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        // Merge known Copilot models that the API may not yet advertise.
        // Source: https://docs.github.com/en/copilot/using-github-copilot/ai-models/changing-the-ai-model-for-github-copilot-chat
        let known_models: Vec<(&str, &str, usize, usize)> = vec![
            // (id, human name, context_window, max_output)
            ("claude-opus-4.6", "Claude Opus 4.6", 200_000, 64_000),
            ("claude-opus-4.5", "Claude Opus 4.5", 200_000, 64_000),
            ("claude-opus-41", "Claude Opus 4.1", 200_000, 64_000),
            ("claude-sonnet-4.5", "Claude Sonnet 4.5", 200_000, 64_000),
            ("claude-sonnet-4", "Claude Sonnet 4", 200_000, 64_000),
            ("claude-haiku-4.5", "Claude Haiku 4.5", 200_000, 64_000),
            ("gpt-5.2", "GPT-5.2", 400_000, 128_000),
            ("gpt-5.2-codex", "GPT-5.2-Codex", 400_000, 128_000),
            ("gpt-5.1", "GPT-5.1", 400_000, 128_000),
            ("gpt-5.1-codex", "GPT-5.1-Codex", 264_000, 64_000),
            ("gpt-5.1-codex-mini", "GPT-5.1-Codex-Mini", 264_000, 64_000),
            ("gpt-5.1-codex-max", "GPT-5.1-Codex-Max", 264_000, 64_000),
            ("gpt-5", "GPT-5", 400_000, 128_000),
            ("gpt-5-codex", "GPT-5-Codex", 264_000, 64_000),
            ("gpt-5-mini", "GPT-5 mini", 264_000, 64_000),
            ("gpt-4.1", "GPT-4.1", 128_000, 32_768),
            ("gpt-4o", "GPT-4o", 128_000, 16_384),
            ("gemini-2.5-pro", "Gemini 2.5 Pro", 1_000_000, 64_000),
            ("gemini-3-flash", "Gemini 3 Flash", 1_000_000, 64_000),
            ("gemini-3-pro", "Gemini 3 Pro", 1_000_000, 64_000),
            ("grok-code-fast-1", "Grok Code Fast 1", 128_000, 32_768),
            ("raptor-mini", "Raptor mini", 264_000, 64_000),
        ];

        let existing_ids: std::collections::HashSet<String> =
            models.iter().map(|m| m.id.clone()).collect();

        for (id, name, ctx, max_out) in known_models {
            if !existing_ids.contains(id) {
                models.push(ModelInfo {
                    id: id.to_string(),
                    name: name.to_string(),
                    provider: self.provider_name.clone(),
                    context_window: ctx,
                    max_output_tokens: Some(max_out),
                    supports_vision: true,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: Some(0.0),
                    output_cost_per_million: Some(0.0),
                });
            }
        }

        Ok(models)
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);
        let is_agent = Self::is_agent_initiated(&request.messages);
        let has_vision = Self::has_vision_input(&request.messages);

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
        if let Some(top_p) = request.top_p {
            body["top_p"] = json!(top_p);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }
        if !request.stop.is_empty() {
            body["stop"] = json!(request.stop);
        }

        let mut req = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .header("Openai-Intent", "conversation-edits")
            .header("User-Agent", Self::user_agent())
            .header("X-Initiator", if is_agent { "agent" } else { "user" });

        if has_vision {
            req = req.header("Copilot-Vision-Request", "true");
        }

        let response = req
            .json(&body)
            .send()
            .await
            .context("Failed to send Copilot request")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Copilot response")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<CopilotErrorResponse>(&text) {
                let message = err
                    .error
                    .and_then(|detail| {
                        detail.message.map(|msg| {
                            if let Some(code) = detail.code {
                                format!("{} ({})", msg, code)
                            } else {
                                msg
                            }
                        })
                    })
                    .or(err.message)
                    .unwrap_or_else(|| "Unknown Copilot API error".to_string());
                anyhow::bail!("Copilot API error: {}", message);
            }
            anyhow::bail!("Copilot API error: {} {}", status, text);
        }

        let response: CopilotResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse Copilot response: {}",
            &text[..text.len().min(200)]
        ))?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices"))?;

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
                prompt_tokens: response
                    .usage
                    .as_ref()
                    .map(|u| u.prompt_tokens)
                    .unwrap_or(0),
                completion_tokens: response
                    .usage
                    .as_ref()
                    .map(|u| u.completion_tokens)
                    .unwrap_or(0),
                total_tokens: response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
                ..Default::default()
            },
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        // For now, keep behavior aligned with other non-streaming providers.
        let response = self.complete(request).await?;
        let text = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");

        Ok(Box::pin(futures::stream::once(async move {
            StreamChunk::Text(text)
        })))
    }
}

pub fn normalize_enterprise_domain(input: &str) -> String {
    input
        .trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_string()
}

pub fn enterprise_base_url(enterprise_url: &str) -> String {
    format!(
        "https://copilot-api.{}",
        normalize_enterprise_domain(enterprise_url)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_enterprise_domain_handles_scheme_and_trailing_slash() {
        assert_eq!(
            normalize_enterprise_domain("https://company.ghe.com/"),
            "company.ghe.com"
        );
        assert_eq!(
            normalize_enterprise_domain("http://company.ghe.com"),
            "company.ghe.com"
        );
        assert_eq!(
            normalize_enterprise_domain("company.ghe.com"),
            "company.ghe.com"
        );
    }

    #[test]
    fn enterprise_base_url_uses_copilot_api_subdomain() {
        assert_eq!(
            enterprise_base_url("https://company.ghe.com/"),
            "https://copilot-api.company.ghe.com"
        );
    }
}
