//! Z.AI provider implementation (direct API)
//!
//! GLM-5, GLM-4.7, and other Z.AI foundation models via api.z.ai.
//! Z.AI (formerly ZhipuAI) offers OpenAI-compatible chat completions with
//! reasoning/thinking support via the `reasoning_content` field.

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

pub struct ZaiProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl std::fmt::Debug for ZaiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZaiProvider")
            .field("base_url", &self.base_url)
            .field("api_key", &"<REDACTED>")
            .finish()
    }
}

impl ZaiProvider {
    pub fn with_base_url(api_key: String, base_url: String) -> Result<Self> {
        tracing::debug!(
            provider = "zai",
            base_url = %base_url,
            api_key_len = api_key.len(),
            "Creating Z.AI provider with custom base URL"
        );
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
        })
    }

    fn convert_messages(messages: &[Message], include_reasoning_content: bool) -> Vec<Value> {
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
                                    ..
                                } => {
                                    // Z.AI request schema expects assistant.tool_calls[*].function.arguments
                                    // to be a JSON-format string. Normalize to a valid JSON string.
                                    let args_string = serde_json::from_str::<Value>(arguments)
                                        .map(|parsed| {
                                            serde_json::to_string(&parsed)
                                                .unwrap_or_else(|_| "{}".to_string())
                                        })
                                        .unwrap_or_else(|_| {
                                            json!({"input": arguments}).to_string()
                                        });
                                    Some(json!({
                                        "id": id,
                                        "type": "function",
                                        "function": {
                                            "name": name,
                                            "arguments": args_string
                                        }
                                    }))
                                }
                                _ => None,
                            })
                            .collect();

                        let mut msg_json = json!({
                            "role": "assistant",
                            "content": if text.is_empty() { "".to_string() } else { text },
                        });
                        if include_reasoning_content {
                            let reasoning: String = msg
                                .content
                                .iter()
                                .filter_map(|p| match p {
                                    ContentPart::Thinking { text } => Some(text.clone()),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join("");
                            if !reasoning.is_empty() {
                                msg_json["reasoning_content"] = json!(reasoning);
                            }
                        }
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

    fn model_supports_tool_stream(model: &str) -> bool {
        model.contains("glm-5") || model.contains("glm-4.7") || model.contains("glm-4.6")
    }

    fn preview_text(text: &str, max_chars: usize) -> &str {
        if max_chars == 0 {
            return "";
        }
        if let Some((idx, _)) = text.char_indices().nth(max_chars) {
            &text[..idx]
        } else {
            text
        }
    }
}

#[derive(Debug, Deserialize)]
struct ZaiResponse {
    choices: Vec<ZaiChoice>,
    #[serde(default)]
    usage: Option<ZaiUsage>,
}

#[derive(Debug, Deserialize)]
struct ZaiChoice {
    message: ZaiMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ZaiMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ZaiToolCall>>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ZaiToolCall {
    id: String,
    function: ZaiFunction,
}

#[derive(Debug, Deserialize)]
struct ZaiFunction {
    name: String,
    arguments: Value,
}

#[derive(Debug, Deserialize)]
struct ZaiUsage {
    #[serde(default)]
    prompt_tokens: usize,
    #[serde(default)]
    completion_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
    #[serde(default)]
    prompt_tokens_details: Option<ZaiPromptTokensDetails>,
}

#[derive(Debug, Deserialize)]
struct ZaiPromptTokensDetails {
    #[serde(default)]
    cached_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct ZaiError {
    error: ZaiErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ZaiErrorDetail {
    message: String,
    #[serde(default, rename = "type")]
    error_type: Option<String>,
}

// SSE stream types
#[derive(Debug, Deserialize)]
struct ZaiStreamResponse {
    choices: Vec<ZaiStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct ZaiStreamChoice {
    delta: ZaiStreamDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ZaiStreamDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    reasoning_content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ZaiStreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct ZaiStreamToolCall {
    #[serde(default)]
    id: Option<String>,
    function: Option<ZaiStreamFunction>,
}

#[derive(Debug, Deserialize)]
struct ZaiStreamFunction {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<Value>,
}

#[async_trait]
impl Provider for ZaiProvider {
    fn name(&self) -> &str {
        "zai"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "glm-5".to_string(),
                name: "GLM-5".to_string(),
                provider: "zai".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-4.7".to_string(),
                name: "GLM-4.7".to_string(),
                provider: "zai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-4.7-flash".to_string(),
                name: "GLM-4.7 Flash".to_string(),
                provider: "zai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-4.6".to_string(),
                name: "GLM-4.6".to_string(),
                provider: "zai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-4.5".to_string(),
                name: "GLM-4.5".to_string(),
                provider: "zai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(96_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        // Compatibility-first mode: omit historical reasoning_content from
        // request messages to avoid strict parameter validation errors on some
        // endpoint variants.
        let messages = Self::convert_messages(&request.messages, false);
        let tools = Self::convert_tools(&request.tools);

        // GLM-5 and GLM-4.7 default to temperature 1.0
        let temperature = request.temperature.unwrap_or(1.0);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "temperature": temperature,
        });

        // Keep thinking enabled, but avoid provider-specific sub-fields that
        // may be rejected by stricter API variants.
        body["thinking"] = json!({
            "type": "enabled"
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(model = %request.model, "Z.AI request");

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Z.AI")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Z.AI response")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<ZaiError>(&text) {
                anyhow::bail!(
                    "Z.AI API error: {} ({:?})",
                    err.error.message,
                    err.error.error_type
                );
            }
            anyhow::bail!("Z.AI API error: {} {}", status, text);
        }

        let response: ZaiResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse Z.AI response: {}",
            Self::preview_text(&text, 200)
        ))?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices in Z.AI response"))?;

        // Log thinking/reasoning content if present
        if let Some(ref reasoning) = choice.message.reasoning_content {
            if !reasoning.is_empty() {
                tracing::info!(
                    reasoning_len = reasoning.len(),
                    "Z.AI reasoning content received"
                );
            }
        }

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        // Emit thinking content as a Thinking part
        if let Some(ref reasoning) = choice.message.reasoning_content {
            if !reasoning.is_empty() {
                content.push(ContentPart::Thinking {
                    text: reasoning.clone(),
                });
            }
        }

        if let Some(text) = &choice.message.content {
            if !text.is_empty() {
                content.push(ContentPart::Text { text: text.clone() });
            }
        }

        if let Some(tool_calls) = &choice.message.tool_calls {
            has_tool_calls = !tool_calls.is_empty();
            for tc in tool_calls {
                // Z.AI returns arguments as an object; serialize to string for our ContentPart
                let arguments = match &tc.function.arguments {
                    Value::String(s) => s.clone(),
                    other => serde_json::to_string(other).unwrap_or_default(),
                };
                content.push(ContentPart::ToolCall {
                    id: tc.id.clone(),
                    name: tc.function.name.clone(),
                    arguments,
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
                Some("sensitive") => FinishReason::ContentFilter,
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
                cache_read_tokens: response
                    .usage
                    .as_ref()
                    .and_then(|u| u.prompt_tokens_details.as_ref())
                    .map(|d| d.cached_tokens)
                    .filter(|&t| t > 0),
                cache_write_tokens: None,
            },
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        // Compatibility-first mode: omit historical reasoning_content from
        // request messages to avoid strict parameter validation errors on some
        // endpoint variants.
        let messages = Self::convert_messages(&request.messages, false);
        let tools = Self::convert_tools(&request.tools);

        let temperature = request.temperature.unwrap_or(1.0);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "temperature": temperature,
            "stream": true,
        });

        body["thinking"] = json!({
            "type": "enabled"
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
            if Self::model_supports_tool_stream(&request.model) {
                // Enable streaming tool calls only on known-compatible models.
                body["tool_stream"] = json!(true);
            }
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(model = %request.model, "Z.AI streaming request");

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Z.AI")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<ZaiError>(&text) {
                anyhow::bail!(
                    "Z.AI API error: {} ({:?})",
                    err.error.message,
                    err.error.error_type
                );
            }
            anyhow::bail!("Z.AI streaming error: {} {}", status, text);
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

                            if line == "data: [DONE]" {
                                if !text_buf.is_empty() {
                                    chunks.push(StreamChunk::Text(std::mem::take(&mut text_buf)));
                                }
                                chunks.push(StreamChunk::Done { usage: None });
                                continue;
                            }
                            if let Some(data) = line.strip_prefix("data: ") {
                                if let Ok(parsed) = serde_json::from_str::<ZaiStreamResponse>(data)
                                {
                                    if let Some(choice) = parsed.choices.first() {
                                        // Reasoning content streamed as text (prefixed for TUI rendering)
                                        if let Some(ref reasoning) = choice.delta.reasoning_content
                                        {
                                            if !reasoning.is_empty() {
                                                text_buf.push_str(reasoning);
                                            }
                                        }
                                        if let Some(ref content) = choice.delta.content {
                                            text_buf.push_str(content);
                                        }
                                        // Streaming tool calls
                                        if let Some(ref tool_calls) = choice.delta.tool_calls {
                                            if !text_buf.is_empty() {
                                                chunks.push(StreamChunk::Text(std::mem::take(
                                                    &mut text_buf,
                                                )));
                                            }
                                            for tc in tool_calls {
                                                if let Some(ref func) = tc.function {
                                                    if let Some(ref name) = func.name {
                                                        // New tool call starting
                                                        chunks.push(StreamChunk::ToolCallStart {
                                                            id: tc.id.clone().unwrap_or_default(),
                                                            name: name.clone(),
                                                        });
                                                    }
                                                    if let Some(ref args) = func.arguments {
                                                        let delta = match args {
                                                            Value::String(s) => s.clone(),
                                                            other => serde_json::to_string(other)
                                                                .unwrap_or_default(),
                                                        };
                                                        if !delta.is_empty() {
                                                            chunks.push(
                                                                StreamChunk::ToolCallDelta {
                                                                    id: tc
                                                                        .id
                                                                        .clone()
                                                                        .unwrap_or_default(),
                                                                    arguments_delta: delta,
                                                                },
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        // finish_reason signals end of a tool call or completion
                                        if let Some(ref reason) = choice.finish_reason {
                                            if !text_buf.is_empty() {
                                                chunks.push(StreamChunk::Text(std::mem::take(
                                                    &mut text_buf,
                                                )));
                                            }
                                            if reason == "tool_calls" {
                                                // Emit ToolCallEnd for the last tool call
                                                if let Some(ref tcs) = choice.delta.tool_calls {
                                                    if let Some(tc) = tcs.last() {
                                                        chunks.push(StreamChunk::ToolCallEnd {
                                                            id: tc.id.clone().unwrap_or_default(),
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
    fn convert_messages_serializes_tool_arguments_as_json_string() {
        let messages = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "get_weather".to_string(),
                arguments: "{\"city\":\"Beijing\".. }".to_string(),
                thought_signature: None,
            }],
        }];

        let converted = ZaiProvider::convert_messages(&messages, true);
        let args = converted[0]["tool_calls"][0]["function"]["arguments"]
            .as_str()
            .expect("arguments must be a string");

        assert_eq!(args, "{\"city\":\"Beijing\"}");
    }

    #[test]
    fn convert_messages_wraps_invalid_tool_arguments_as_json_string() {
        let messages = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "get_weather".to_string(),
                arguments: "city=Beijing".to_string(),
                thought_signature: None,
            }],
        }];

        let converted = ZaiProvider::convert_messages(&messages, true);
        let args = converted[0]["tool_calls"][0]["function"]["arguments"]
            .as_str()
            .expect("arguments must be a string");
        let parsed: Value = serde_json::from_str(args).expect("arguments must contain valid JSON");

        assert_eq!(parsed, json!({"input": "city=Beijing"}));
    }

    #[test]
    fn preview_text_truncates_on_char_boundary() {
        let text = "a😀b";
        assert_eq!(ZaiProvider::preview_text(text, 2), "a😀");
    }
}
