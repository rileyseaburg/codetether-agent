//! Amazon Bedrock provider implementation using the Converse API
//!
//! Supports all Bedrock foundation models via API Key bearer token auth.
//! Uses the native Bedrock Converse API format.
//! Dynamically discovers available models via the Bedrock ListFoundationModels
//! and ListInferenceProfiles APIs.
//! Reference: https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_Converse.html

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

const DEFAULT_REGION: &str = "us-east-1";

pub struct BedrockProvider {
    client: Client,
    api_key: String,
    region: String,
}

impl std::fmt::Debug for BedrockProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BedrockProvider")
            .field("api_key", &"<REDACTED>")
            .field("region", &self.region)
            .finish()
    }
}

impl BedrockProvider {
    pub fn new(api_key: String) -> Result<Self> {
        Self::with_region(api_key, DEFAULT_REGION.to_string())
    }

    pub fn with_region(api_key: String, region: String) -> Result<Self> {
        tracing::debug!(
            provider = "bedrock",
            region = %region,
            api_key_len = api_key.len(),
            "Creating Bedrock provider"
        );
        Ok(Self {
            client: Client::new(),
            api_key,
            region,
        })
    }

    fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("Bedrock API key is empty");
        }
        Ok(())
    }

    fn base_url(&self) -> String {
        format!("https://bedrock-runtime.{}.amazonaws.com", self.region)
    }

    /// Management API URL (for listing models, not inference)
    fn management_url(&self) -> String {
        format!("https://bedrock.{}.amazonaws.com", self.region)
    }

    /// Resolve a short model alias to the full Bedrock model ID.
    /// Allows users to specify e.g. "claude-sonnet-4" instead of
    /// "us.anthropic.claude-sonnet-4-20250514-v1:0".
    fn resolve_model_id(model: &str) -> &str {
        match model {
            // --- Anthropic Claude (verified via AWS CLI) ---
            "claude-opus-4.6" | "claude-4.6-opus" => "us.anthropic.claude-opus-4-6-v1",
            "claude-opus-4.5" | "claude-4.5-opus" => "us.anthropic.claude-opus-4-5-20251101-v1:0",
            "claude-opus-4.1" | "claude-4.1-opus" => "us.anthropic.claude-opus-4-1-20250805-v1:0",
            "claude-opus-4" | "claude-4-opus" => "us.anthropic.claude-opus-4-20250514-v1:0",
            "claude-sonnet-4.5" | "claude-4.5-sonnet" => {
                "us.anthropic.claude-sonnet-4-5-20250929-v1:0"
            }
            "claude-sonnet-4" | "claude-4-sonnet" => "us.anthropic.claude-sonnet-4-20250514-v1:0",
            "claude-haiku-4.5" | "claude-4.5-haiku" => {
                "us.anthropic.claude-haiku-4-5-20251001-v1:0"
            }
            "claude-3.7-sonnet" | "claude-sonnet-3.7" => {
                "us.anthropic.claude-3-7-sonnet-20250219-v1:0"
            }
            "claude-3.5-sonnet-v2" | "claude-sonnet-3.5-v2" => {
                "us.anthropic.claude-3-5-sonnet-20241022-v2:0"
            }
            "claude-3.5-haiku" | "claude-haiku-3.5" => {
                "us.anthropic.claude-3-5-haiku-20241022-v1:0"
            }
            "claude-3.5-sonnet" | "claude-sonnet-3.5" => {
                "us.anthropic.claude-3-5-sonnet-20240620-v1:0"
            }
            "claude-3-opus" | "claude-opus-3" => "us.anthropic.claude-3-opus-20240229-v1:0",
            "claude-3-haiku" | "claude-haiku-3" => "us.anthropic.claude-3-haiku-20240307-v1:0",
            "claude-3-sonnet" | "claude-sonnet-3" => "us.anthropic.claude-3-sonnet-20240229-v1:0",

            // --- Amazon Nova ---
            "nova-pro" => "amazon.nova-pro-v1:0",
            "nova-lite" => "amazon.nova-lite-v1:0",
            "nova-micro" => "amazon.nova-micro-v1:0",
            "nova-premier" => "us.amazon.nova-premier-v1:0",

            // --- Meta Llama ---
            "llama-4-maverick" | "llama4-maverick" => "us.meta.llama4-maverick-17b-instruct-v1:0",
            "llama-4-scout" | "llama4-scout" => "us.meta.llama4-scout-17b-instruct-v1:0",
            "llama-3.3-70b" | "llama3.3-70b" => "us.meta.llama3-3-70b-instruct-v1:0",
            "llama-3.2-90b" | "llama3.2-90b" => "us.meta.llama3-2-90b-instruct-v1:0",
            "llama-3.2-11b" | "llama3.2-11b" => "us.meta.llama3-2-11b-instruct-v1:0",
            "llama-3.2-3b" | "llama3.2-3b" => "us.meta.llama3-2-3b-instruct-v1:0",
            "llama-3.2-1b" | "llama3.2-1b" => "us.meta.llama3-2-1b-instruct-v1:0",
            "llama-3.1-70b" | "llama3.1-70b" => "us.meta.llama3-1-70b-instruct-v1:0",
            "llama-3.1-8b" | "llama3.1-8b" => "us.meta.llama3-1-8b-instruct-v1:0",
            "llama-3-70b" | "llama3-70b" => "us.meta.llama3-70b-instruct-v1:0",
            "llama-3-8b" | "llama3-8b" => "us.meta.llama3-8b-instruct-v1:0",

            // --- Mistral ---
            "mistral-large-3" | "mistral-large" => "us.mistral.mistral-large-3-675b-instruct",
            "mistral-large-2402" => "us.mistral.mistral-large-2402-v1:0",
            "mistral-small" => "us.mistral.mistral-small-2402-v1:0",
            "mixtral-8x7b" => "us.mistral.mixtral-8x7b-instruct-v0:1",
            "pixtral-large" => "us.mistral.pixtral-large-2502-v1:0",
            "magistral-small" => "us.mistral.magistral-small-2509",

            // --- DeepSeek ---
            "deepseek-r1" => "us.deepseek.r1-v1:0",
            "deepseek-v3" | "deepseek-v3.2" => "us.deepseek.v3.2",

            // --- Cohere ---
            "command-r" => "us.cohere.command-r-v1:0",
            "command-r-plus" => "us.cohere.command-r-plus-v1:0",

            // --- Qwen ---
            "qwen3-32b" => "us.qwen.qwen3-32b-v1:0",
            "qwen3-coder" | "qwen3-coder-next" => "us.qwen.qwen3-coder-next",
            "qwen3-coder-30b" => "us.qwen.qwen3-coder-30b-a3b-v1:0",

            // --- Google Gemma ---
            "gemma-3-27b" => "us.google.gemma-3-27b-it",
            "gemma-3-12b" => "us.google.gemma-3-12b-it",
            "gemma-3-4b" => "us.google.gemma-3-4b-it",

            // --- Moonshot / Kimi ---
            "kimi-k2" | "kimi-k2-thinking" => "us.moonshot.kimi-k2-thinking",
            "kimi-k2.5" => "us.moonshotai.kimi-k2.5",

            // --- AI21 Jamba ---
            "jamba-1.5-large" => "us.ai21.jamba-1-5-large-v1:0",
            "jamba-1.5-mini" => "us.ai21.jamba-1-5-mini-v1:0",

            // --- MiniMax ---
            "minimax-m2" => "us.minimax.minimax-m2",
            "minimax-m2.1" => "us.minimax.minimax-m2.1",

            // --- NVIDIA ---
            "nemotron-nano-30b" => "us.nvidia.nemotron-nano-3-30b",
            "nemotron-nano-12b" => "us.nvidia.nemotron-nano-12b-v2",
            "nemotron-nano-9b" => "us.nvidia.nemotron-nano-9b-v2",

            // --- Z.AI / GLM ---
            "glm-4.7" => "us.zai.glm-4.7",
            "glm-4.7-flash" => "us.zai.glm-4.7-flash",

            // Pass through full model IDs unchanged
            other => other,
        }
    }

    /// Dynamically discover available models from the Bedrock API.
    /// Merges foundation models with cross-region inference profiles.
    async fn discover_models(&self) -> Result<Vec<ModelInfo>> {
        let mut models: HashMap<String, ModelInfo> = HashMap::new();

        // 1) Fetch foundation models
        let fm_url = format!("{}/foundation-models", self.management_url());
        let fm_resp = self
            .client
            .get(&fm_url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        if let Ok(resp) = fm_resp {
            if resp.status().is_success() {
                if let Ok(data) = resp.json::<Value>().await {
                    if let Some(summaries) = data.get("modelSummaries").and_then(|v| v.as_array()) {
                        for m in summaries {
                            let model_id = m.get("modelId").and_then(|v| v.as_str()).unwrap_or("");
                            let model_name =
                                m.get("modelName").and_then(|v| v.as_str()).unwrap_or("");
                            let provider_name =
                                m.get("providerName").and_then(|v| v.as_str()).unwrap_or("");

                            let output_modalities: Vec<&str> = m
                                .get("outputModalities")
                                .and_then(|v| v.as_array())
                                .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                                .unwrap_or_default();

                            let input_modalities: Vec<&str> = m
                                .get("inputModalities")
                                .and_then(|v| v.as_array())
                                .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                                .unwrap_or_default();

                            let inference_types: Vec<&str> = m
                                .get("inferenceTypesSupported")
                                .and_then(|v| v.as_array())
                                .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                                .unwrap_or_default();

                            // Only include TEXT output models with ON_DEMAND or INFERENCE_PROFILE inference
                            if !output_modalities.contains(&"TEXT")
                                || (!inference_types.contains(&"ON_DEMAND")
                                    && !inference_types.contains(&"INFERENCE_PROFILE"))
                            {
                                continue;
                            }

                            // Skip non-chat models
                            let name_lower = model_name.to_lowercase();
                            if name_lower.contains("rerank")
                                || name_lower.contains("embed")
                                || name_lower.contains("safeguard")
                                || name_lower.contains("sonic")
                                || name_lower.contains("pegasus")
                            {
                                continue;
                            }

                            let streaming = m
                                .get("responseStreamingSupported")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);
                            let vision = input_modalities.contains(&"IMAGE");

                            // Non-Amazon models need us. cross-region prefix
                            let actual_id = if model_id.starts_with("amazon.") {
                                model_id.to_string()
                            } else {
                                format!("us.{}", model_id)
                            };

                            let display_name = format!("{} (Bedrock)", model_name);

                            models.insert(
                                actual_id.clone(),
                                ModelInfo {
                                    id: actual_id,
                                    name: display_name,
                                    provider: "bedrock".to_string(),
                                    context_window: Self::estimate_context_window(
                                        model_id,
                                        provider_name,
                                    ),
                                    max_output_tokens: Some(Self::estimate_max_output(
                                        model_id,
                                        provider_name,
                                    )),
                                    supports_vision: vision,
                                    supports_tools: true,
                                    supports_streaming: streaming,
                                    input_cost_per_million: None,
                                    output_cost_per_million: None,
                                },
                            );
                        }
                    }
                }
            }
        }

        // 2) Fetch cross-region inference profiles (adds models like Claude Sonnet 4,
        //    Llama 3.1/3.2/3.3/4, DeepSeek R1, etc. that aren't in foundation models)
        let ip_url = format!(
            "{}/inference-profiles?typeEquals=SYSTEM_DEFINED&maxResults=200",
            self.management_url()
        );
        let ip_resp = self
            .client
            .get(&ip_url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        if let Ok(resp) = ip_resp {
            if resp.status().is_success() {
                if let Ok(data) = resp.json::<Value>().await {
                    if let Some(profiles) = data
                        .get("inferenceProfileSummaries")
                        .and_then(|v| v.as_array())
                    {
                        for p in profiles {
                            let pid = p
                                .get("inferenceProfileId")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
                            let pname = p
                                .get("inferenceProfileName")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");

                            // Only US cross-region profiles
                            if !pid.starts_with("us.") {
                                continue;
                            }

                            // Skip already-discovered models
                            if models.contains_key(pid) {
                                continue;
                            }

                            // Skip non-text models
                            let name_lower = pname.to_lowercase();
                            if name_lower.contains("image")
                                || name_lower.contains("stable ")
                                || name_lower.contains("upscale")
                                || name_lower.contains("embed")
                                || name_lower.contains("marengo")
                                || name_lower.contains("outpaint")
                                || name_lower.contains("inpaint")
                                || name_lower.contains("erase")
                                || name_lower.contains("recolor")
                                || name_lower.contains("replace")
                                || name_lower.contains("style ")
                                || name_lower.contains("background")
                                || name_lower.contains("sketch")
                                || name_lower.contains("control")
                                || name_lower.contains("transfer")
                                || name_lower.contains("sonic")
                                || name_lower.contains("pegasus")
                                || name_lower.contains("rerank")
                            {
                                continue;
                            }

                            // Guess vision from known model families
                            let vision = pid.contains("llama3-2-11b")
                                || pid.contains("llama3-2-90b")
                                || pid.contains("pixtral")
                                || pid.contains("claude-3")
                                || pid.contains("claude-sonnet-4")
                                || pid.contains("claude-opus-4")
                                || pid.contains("claude-haiku-4");

                            let display_name = pname.replace("US ", "");
                            let display_name = format!("{} (Bedrock)", display_name.trim());

                            // Extract provider hint from model ID
                            let provider_hint = pid
                                .strip_prefix("us.")
                                .unwrap_or(pid)
                                .split('.')
                                .next()
                                .unwrap_or("");

                            models.insert(
                                pid.to_string(),
                                ModelInfo {
                                    id: pid.to_string(),
                                    name: display_name,
                                    provider: "bedrock".to_string(),
                                    context_window: Self::estimate_context_window(
                                        pid,
                                        provider_hint,
                                    ),
                                    max_output_tokens: Some(Self::estimate_max_output(
                                        pid,
                                        provider_hint,
                                    )),
                                    supports_vision: vision,
                                    supports_tools: true,
                                    supports_streaming: true,
                                    input_cost_per_million: None,
                                    output_cost_per_million: None,
                                },
                            );
                        }
                    }
                }
            }
        }

        let mut result: Vec<ModelInfo> = models.into_values().collect();
        result.sort_by(|a, b| a.id.cmp(&b.id));

        tracing::info!(
            provider = "bedrock",
            model_count = result.len(),
            "Discovered Bedrock models dynamically"
        );

        Ok(result)
    }

    /// Estimate context window size based on model family
    fn estimate_context_window(model_id: &str, provider: &str) -> usize {
        let id = model_id.to_lowercase();
        if id.contains("anthropic") || id.contains("claude") {
            200_000
        } else if id.contains("nova-pro") || id.contains("nova-lite") || id.contains("nova-premier")
        {
            300_000
        } else if id.contains("nova-micro") || id.contains("nova-2") {
            128_000
        } else if id.contains("deepseek") {
            128_000
        } else if id.contains("llama4") {
            256_000
        } else if id.contains("llama3") {
            128_000
        } else if id.contains("mistral-large-3") || id.contains("magistral") {
            128_000
        } else if id.contains("mistral") {
            32_000
        } else if id.contains("qwen") {
            128_000
        } else if id.contains("kimi") {
            128_000
        } else if id.contains("jamba") {
            256_000
        } else if id.contains("glm") {
            128_000
        } else if id.contains("minimax") {
            128_000
        } else if id.contains("gemma") {
            128_000
        } else if id.contains("cohere") || id.contains("command") {
            128_000
        } else if id.contains("nemotron") {
            128_000
        } else if provider.to_lowercase().contains("amazon") {
            128_000
        } else {
            32_000
        }
    }

    /// Estimate max output tokens based on model family
    fn estimate_max_output(model_id: &str, _provider: &str) -> usize {
        let id = model_id.to_lowercase();
        if id.contains("claude-opus-4-6") {
            32_000
        } else if id.contains("claude-opus-4-5") {
            32_000
        } else if id.contains("claude-opus-4-1") {
            32_000
        } else if id.contains("claude-sonnet-4-5")
            || id.contains("claude-sonnet-4")
            || id.contains("claude-3-7")
        {
            64_000
        } else if id.contains("claude-haiku-4-5") {
            16_384
        } else if id.contains("claude-opus-4") {
            32_000
        } else if id.contains("claude") {
            8_192
        } else if id.contains("nova") {
            5_000
        } else if id.contains("deepseek") {
            16_384
        } else if id.contains("llama4") {
            16_384
        } else if id.contains("llama") {
            4_096
        } else if id.contains("mistral-large-3") {
            16_384
        } else if id.contains("mistral") || id.contains("mixtral") {
            8_192
        } else if id.contains("qwen") {
            8_192
        } else if id.contains("kimi") {
            8_192
        } else if id.contains("jamba") {
            4_096
        } else {
            4_096
        }
    }

    /// Convert our generic messages to Bedrock Converse API format.
    ///
    /// Bedrock Converse uses:
    /// - system prompt as a top-level "system" array
    /// - messages with "role" and "content" array
    /// - tool_use blocks in assistant content
    /// - toolResult blocks in user content
    ///
    /// IMPORTANT: Bedrock requires strict role alternation (user/assistant).
    /// Consecutive Role::Tool messages must be merged into a single "user"
    /// message so all toolResult blocks for a given assistant turn appear
    /// together. Consecutive same-role messages are also merged to prevent
    /// validation errors.
    fn convert_messages(messages: &[Message]) -> (Vec<Value>, Vec<Value>) {
        let mut system_parts: Vec<Value> = Vec::new();
        let mut api_messages: Vec<Value> = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    let text: String = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    system_parts.push(json!({"text": text}));
                }
                Role::User => {
                    let mut content_parts: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                if !text.is_empty() {
                                    content_parts.push(json!({"text": text}));
                                }
                            }
                            _ => {}
                        }
                    }
                    if !content_parts.is_empty() {
                        // Merge into previous user message if the last message is also "user"
                        if let Some(last) = api_messages.last_mut() {
                            if last.get("role").and_then(|r| r.as_str()) == Some("user") {
                                if let Some(arr) =
                                    last.get_mut("content").and_then(|c| c.as_array_mut())
                                {
                                    arr.extend(content_parts);
                                    continue;
                                }
                            }
                        }
                        api_messages.push(json!({
                            "role": "user",
                            "content": content_parts
                        }));
                    }
                }
                Role::Assistant => {
                    let mut content_parts: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                if !text.is_empty() {
                                    content_parts.push(json!({"text": text}));
                                }
                            }
                            ContentPart::ToolCall {
                                id,
                                name,
                                arguments,
                            } => {
                                let input: Value = serde_json::from_str(arguments)
                                    .unwrap_or_else(|_| json!({"raw": arguments}));
                                content_parts.push(json!({
                                    "toolUse": {
                                        "toolUseId": id,
                                        "name": name,
                                        "input": input
                                    }
                                }));
                            }
                            _ => {}
                        }
                    }
                    if content_parts.is_empty() {
                        content_parts.push(json!({"text": ""}));
                    }
                    // Merge into previous assistant message if consecutive
                    if let Some(last) = api_messages.last_mut() {
                        if last.get("role").and_then(|r| r.as_str()) == Some("assistant") {
                            if let Some(arr) =
                                last.get_mut("content").and_then(|c| c.as_array_mut())
                            {
                                arr.extend(content_parts);
                                continue;
                            }
                        }
                    }
                    api_messages.push(json!({
                        "role": "assistant",
                        "content": content_parts
                    }));
                }
                Role::Tool => {
                    // Tool results must be in a "user" message with toolResult blocks.
                    // Merge into the previous user message if one exists (handles
                    // consecutive Tool messages being collapsed into one user turn).
                    let mut content_parts: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        if let ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        } = part
                        {
                            content_parts.push(json!({
                                "toolResult": {
                                    "toolUseId": tool_call_id,
                                    "content": [{"text": content}],
                                    "status": "success"
                                }
                            }));
                        }
                    }
                    if !content_parts.is_empty() {
                        // Merge into previous user message (from earlier Tool messages)
                        if let Some(last) = api_messages.last_mut() {
                            if last.get("role").and_then(|r| r.as_str()) == Some("user") {
                                if let Some(arr) =
                                    last.get_mut("content").and_then(|c| c.as_array_mut())
                                {
                                    arr.extend(content_parts);
                                    continue;
                                }
                            }
                        }
                        api_messages.push(json!({
                            "role": "user",
                            "content": content_parts
                        }));
                    }
                }
            }
        }

        (system_parts, api_messages)
    }

    fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "toolSpec": {
                        "name": t.name,
                        "description": t.description,
                        "inputSchema": {
                            "json": t.parameters
                        }
                    }
                })
            })
            .collect()
    }
}

/// Bedrock Converse API response types

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseResponse {
    output: ConverseOutput,
    #[serde(default)]
    stop_reason: Option<String>,
    #[serde(default)]
    usage: Option<ConverseUsage>,
}

#[derive(Debug, Deserialize)]
struct ConverseOutput {
    message: ConverseMessage,
}

#[derive(Debug, Deserialize)]
struct ConverseMessage {
    #[allow(dead_code)]
    role: String,
    content: Vec<ConverseContent>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ConverseContent {
    Text {
        text: String,
    },
    ToolUse {
        #[serde(rename = "toolUse")]
        tool_use: ConverseToolUse,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseToolUse {
    tool_use_id: String,
    name: String,
    input: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseUsage {
    #[serde(default)]
    input_tokens: usize,
    #[serde(default)]
    output_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct BedrockError {
    message: String,
}

#[async_trait]
impl Provider for BedrockProvider {
    fn name(&self) -> &str {
        "bedrock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_api_key()?;
        self.discover_models().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let model_id = Self::resolve_model_id(&request.model);

        tracing::debug!(
            provider = "bedrock",
            model = %model_id,
            original_model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting Bedrock Converse request"
        );

        self.validate_api_key()?;

        let (system_parts, messages) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let mut body = json!({
            "messages": messages,
        });

        if !system_parts.is_empty() {
            body["system"] = json!(system_parts);
        }

        // inferenceConfig
        let mut inference_config = json!({});
        if let Some(max_tokens) = request.max_tokens {
            inference_config["maxTokens"] = json!(max_tokens);
        } else {
            inference_config["maxTokens"] = json!(8192);
        }
        if let Some(temp) = request.temperature {
            inference_config["temperature"] = json!(temp);
        }
        if let Some(top_p) = request.top_p {
            inference_config["topP"] = json!(top_p);
        }
        body["inferenceConfig"] = inference_config;

        if !tools.is_empty() {
            body["toolConfig"] = json!({"tools": tools});
        }

        // URL-encode the colon in model IDs (e.g. v1:0 -> v1%3A0)
        let encoded_model_id = model_id.replace(':', "%3A");
        let url = format!("{}/model/{}/converse", self.base_url(), encoded_model_id);
        tracing::debug!("Bedrock request URL: {}", url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Bedrock")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Bedrock response")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<BedrockError>(&text) {
                anyhow::bail!("Bedrock API error ({}): {}", status, err.message);
            }
            anyhow::bail!(
                "Bedrock API error: {} {}",
                status,
                &text[..text.len().min(500)]
            );
        }

        let response: ConverseResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse Bedrock response: {}",
            &text[..text.len().min(300)]
        ))?;

        tracing::debug!(
            stop_reason = ?response.stop_reason,
            "Received Bedrock response"
        );

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        for part in &response.output.message.content {
            match part {
                ConverseContent::Text { text } => {
                    if !text.is_empty() {
                        content.push(ContentPart::Text { text: text.clone() });
                    }
                }
                ConverseContent::ToolUse { tool_use } => {
                    has_tool_calls = true;
                    content.push(ContentPart::ToolCall {
                        id: tool_use.tool_use_id.clone(),
                        name: tool_use.name.clone(),
                        arguments: serde_json::to_string(&tool_use.input).unwrap_or_default(),
                    });
                }
            }
        }

        let finish_reason = if has_tool_calls {
            FinishReason::ToolCalls
        } else {
            match response.stop_reason.as_deref() {
                Some("end_turn") | Some("stop") | Some("stop_sequence") => FinishReason::Stop,
                Some("max_tokens") => FinishReason::Length,
                Some("tool_use") => FinishReason::ToolCalls,
                Some("content_filtered") => FinishReason::ContentFilter,
                _ => FinishReason::Stop,
            }
        };

        let usage = response.usage.as_ref();

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage: Usage {
                prompt_tokens: usage.map(|u| u.input_tokens).unwrap_or(0),
                completion_tokens: usage.map(|u| u.output_tokens).unwrap_or(0),
                total_tokens: usage.map(|u| u.total_tokens).unwrap_or(0),
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
        // Fall back to non-streaming for now
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
