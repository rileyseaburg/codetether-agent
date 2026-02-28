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

    /// Discover models dynamically from the Copilot /models API endpoint.
    async fn discover_models_from_api(&self) -> Vec<ModelInfo> {
        let response = match self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", Self::user_agent())
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(provider = %self.provider_name, error = %e, "Failed to fetch Copilot models endpoint");
                return Vec::new();
            }
        };

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            tracing::warn!(
                provider = %self.provider_name,
                status = %status,
                body = %body.chars().take(200).collect::<String>(),
                "Copilot /models endpoint returned non-success"
            );
            return Vec::new();
        }

        let parsed: CopilotModelsResponse = match response.json().await {
            Ok(p) => p,
            Err(e) => {
                tracing::warn!(provider = %self.provider_name, error = %e, "Failed to parse Copilot models response");
                return Vec::new();
            }
        };

        let models: Vec<ModelInfo> = parsed
            .data
            .into_iter()
            .filter(|model| {
                // Skip models explicitly disabled in the picker
                if model.model_picker_enabled == Some(false) {
                    return false;
                }
                // Skip models with a disabled policy state
                if let Some(ref policy) = model.policy
                    && policy.state.as_deref() == Some("disabled")
                {
                    return false;
                }
                true
            })
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
                    max_output_tokens: limits.and_then(|l| l.max_output_tokens).or(Some(16_384)),
                    supports_vision: supports.and_then(|s| s.vision).unwrap_or(false),
                    supports_tools: supports.and_then(|s| s.tool_calls).unwrap_or(true),
                    supports_streaming: supports.and_then(|s| s.streaming).unwrap_or(true),
                    input_cost_per_million: None,
                    output_cost_per_million: None,
                }
            })
            .collect();

        tracing::info!(
            provider = %self.provider_name,
            count = models.len(),
            "Discovered models from Copilot API"
        );
        models
    }

    /// Enrich models with pricing metadata from known premium request multipliers.
    ///
    /// Source: https://docs.github.com/en/copilot/concepts/billing/copilot-requests
    /// Cost model: Premium requests at $0.04/request overflow rate.
    /// We convert multiplier to approximate $/M tokens using ~4K tokens/request avg.
    /// Formula: multiplier * $0.04 / 4K tokens * 1M = multiplier * $10/M tokens.
    fn enrich_with_pricing(&self, models: &mut [ModelInfo]) {
        // (display_name, premium_multiplier)
        let pricing: std::collections::HashMap<&str, (&str, f64)> = [
            ("claude-opus-4.5", ("Claude Opus 4.5", 3.0)),
            ("claude-opus-4.6", ("Claude Opus 4.6", 3.0)),
            ("claude-opus-41", ("Claude Opus 4.1", 10.0)),
            ("claude-sonnet-4-6", ("Claude Sonnet 4.6", 1.0)),
            ("claude-sonnet-4.5", ("Claude Sonnet 4.5", 1.0)),
            ("claude-sonnet-4", ("Claude Sonnet 4", 1.0)),
            ("claude-haiku-4.5", ("Claude Haiku 4.5", 0.33)),
            ("gpt-5.3-codex", ("GPT-5.3-Codex", 1.0)),
            ("gpt-5.2", ("GPT-5.2", 1.0)),
            ("gpt-5.2-codex", ("GPT-5.2-Codex", 1.0)),
            ("gpt-5.1", ("GPT-5.1", 1.0)),
            ("gpt-5.1-codex", ("GPT-5.1-Codex", 1.0)),
            ("gpt-5.1-codex-mini", ("GPT-5.1-Codex-Mini", 0.33)),
            ("gpt-5.1-codex-max", ("GPT-5.1-Codex-Max", 1.0)),
            ("gpt-5", ("GPT-5", 1.0)),
            ("gpt-5-mini", ("GPT-5 mini", 0.0)),
            ("gpt-5-codex", ("GPT-5-Codex", 1.0)),
            ("gpt-4.1", ("GPT-4.1", 0.0)),
            ("gpt-4o", ("GPT-4o", 0.0)),
            ("gemini-2.5-pro", ("Gemini 2.5 Pro", 1.0)),
            ("gemini-3.1-pro-preview", ("Gemini 3.1 Pro Preview", 1.0)),
            (
                "gemini-3.1-pro-preview-customtools",
                ("Gemini 3.1 Pro Preview (Custom Tools)", 1.0),
            ),
            ("gemini-3-flash-preview", ("Gemini 3 Flash Preview", 0.33)),
            ("gemini-3-pro-preview", ("Gemini 3 Pro Preview", 1.0)),
            (
                "gemini-3-pro-image-preview",
                ("Gemini 3 Pro Image Preview", 1.0),
            ),
            ("grok-code-fast-1", ("Grok Code Fast 1", 0.25)),
        ]
        .into_iter()
        .collect();

        for model in models.iter_mut() {
            if let Some((display_name, premium_mult)) = pricing.get(model.id.as_str()) {
                // Set a friendlier display name when the API only returned the raw id
                if model.name == model.id {
                    model.name = display_name.to_string();
                }
                let approx_cost = premium_mult * 10.0;
                model.input_cost_per_million = Some(approx_cost);
                model.output_cost_per_million = Some(approx_cost);
            } else {
                // Unknown Copilot model — assume 1x premium request ($10/M approx)
                if model.input_cost_per_million.is_none() {
                    model.input_cost_per_million = Some(10.0);
                }
                if model.output_cost_per_million.is_none() {
                    model.output_cost_per_million = Some(10.0);
                }
            }
        }
    }

    /// Known models to use as a fallback when the /models API is unreachable.
    fn known_models(&self) -> Vec<ModelInfo> {
        let entries: &[(&str, &str, usize, usize, bool)] = &[
            ("gpt-4o", "GPT-4o", 128_000, 16_384, true),
            ("gpt-4.1", "GPT-4.1", 128_000, 32_768, false),
            ("gpt-5", "GPT-5", 400_000, 128_000, false),
            ("gpt-5-mini", "GPT-5 mini", 264_000, 64_000, false),
            ("claude-sonnet-4", "Claude Sonnet 4", 200_000, 64_000, false),
            (
                "claude-sonnet-4.5",
                "Claude Sonnet 4.5",
                200_000,
                64_000,
                false,
            ),
            (
                "claude-sonnet-4-6",
                "Claude Sonnet 4.6",
                200_000,
                128_000,
                false,
            ),
            (
                "claude-haiku-4.5",
                "Claude Haiku 4.5",
                200_000,
                64_000,
                false,
            ),
            ("gemini-2.5-pro", "Gemini 2.5 Pro", 1_000_000, 64_000, false),
            (
                "gemini-3.1-pro-preview",
                "Gemini 3.1 Pro Preview",
                1_048_576,
                65_536,
                false,
            ),
            (
                "gemini-3.1-pro-preview-customtools",
                "Gemini 3.1 Pro Preview (Custom Tools)",
                1_048_576,
                65_536,
                false,
            ),
            (
                "gemini-3-pro-preview",
                "Gemini 3 Pro Preview",
                1_048_576,
                65_536,
                false,
            ),
            (
                "gemini-3-flash-preview",
                "Gemini 3 Flash Preview",
                1_048_576,
                65_536,
                false,
            ),
            (
                "gemini-3-pro-image-preview",
                "Gemini 3 Pro Image Preview",
                65_536,
                32_768,
                false,
            ),
        ];

        entries
            .iter()
            .map(|(id, name, ctx, max_out, vision)| ModelInfo {
                id: id.to_string(),
                name: name.to_string(),
                provider: self.provider_name.clone(),
                context_window: *ctx,
                max_output_tokens: Some(*max_out),
                supports_vision: *vision,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            })
            .collect()
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
        let mut models = self.discover_models_from_api().await;

        // If API discovery returned nothing, fall back to known models
        if models.is_empty() {
            tracing::info!(provider = %self.provider_name, "No models from API, using known model catalog");
            models = self.known_models();
        }

        // Enrich with pricing metadata from known premium request multipliers
        self.enrich_with_pricing(&mut models);

        // Filter out non-chat models (embeddings, etc.) and legacy dated variants
        models.retain(|m| {
            !m.id.starts_with("text-embedding")
                && !m.id.contains("-embedding-")
                && !is_dated_model_variant(&m.id)
        });

        // Deduplicate by id (API sometimes returns duplicates)
        let mut seen = std::collections::HashSet::new();
        models.retain(|m| seen.insert(m.id.clone()));

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

        if let Some(text) = &choice.message.content
            && !text.is_empty()
        {
            content.push(ContentPart::Text { text: text.clone() });
        }

        if let Some(tool_calls) = &choice.message.tool_calls {
            has_tool_calls = !tool_calls.is_empty();
            for tc in tool_calls {
                content.push(ContentPart::ToolCall {
                    id: tc.id.clone(),
                    name: tc.function.name.clone(),
                    arguments: tc.function.arguments.clone(),
                    thought_signature: None,
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

/// Check if a model ID is a dated variant (e.g. "gpt-4o-2024-05-13") that should
/// be filtered out in favor of the canonical alias (e.g. "gpt-4o").
fn is_dated_model_variant(id: &str) -> bool {
    // Match IDs ending in a YYYY-MM-DD date suffix
    let bytes = id.as_bytes();
    if bytes.len() < 11 {
        return false;
    }
    // Check for "-YYYY-MM-DD" at end
    let tail = &id[id.len() - 11..];
    tail.starts_with('-')
        && tail[1..5].bytes().all(|b| b.is_ascii_digit())
        && tail.as_bytes()[5] == b'-'
        && tail[6..8].bytes().all(|b| b.is_ascii_digit())
        && tail.as_bytes()[8] == b'-'
        && tail[9..11].bytes().all(|b| b.is_ascii_digit())
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

    #[test]
    fn is_dated_model_variant_detects_date_suffix() {
        assert!(is_dated_model_variant("gpt-4o-2024-05-13"));
        assert!(is_dated_model_variant("gpt-4o-2024-08-06"));
        assert!(is_dated_model_variant("gpt-4.1-2025-04-14"));
        assert!(is_dated_model_variant("gpt-4o-mini-2024-07-18"));
        assert!(!is_dated_model_variant("gpt-4o"));
        assert!(!is_dated_model_variant("gpt-5"));
        assert!(!is_dated_model_variant("claude-sonnet-4"));
        assert!(!is_dated_model_variant("gemini-2.5-pro"));
    }

    #[test]
    fn known_models_fallback_is_non_empty() {
        let provider = CopilotProvider::new("test-token".to_string()).unwrap();
        let models = provider.known_models();
        assert!(!models.is_empty());
        // All fallback models should support tools
        assert!(models.iter().all(|m| m.supports_tools));
    }

    #[test]
    fn enrich_with_pricing_sets_costs() {
        let provider = CopilotProvider::new("test-token".to_string()).unwrap();
        let mut models = vec![ModelInfo {
            id: "gpt-4o".to_string(),
            name: "gpt-4o".to_string(),
            provider: "github-copilot".to_string(),
            context_window: 128_000,
            max_output_tokens: Some(16_384),
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: None,
            output_cost_per_million: None,
        }];
        provider.enrich_with_pricing(&mut models);
        // GPT-4o is 0x premium (free), so cost = 0.0
        assert_eq!(models[0].input_cost_per_million, Some(0.0));
        assert_eq!(models[0].output_cost_per_million, Some(0.0));
        // Name should be enriched
        assert_eq!(models[0].name, "GPT-4o");
    }
}
