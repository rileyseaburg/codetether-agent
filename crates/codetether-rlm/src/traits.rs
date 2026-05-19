//! Trait abstractions for host-crate dependencies.
//!
//! `codetether-rlm` is independent and cannot depend on
//! `codetether-agent`. These traits define the narrow interface
//! the RLM code needs. The main crate provides implementations.

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Schema-driven tool definition passed to the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// A tool call returned by the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Completion response from a provider.
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub finish_reason: Option<String>,
    pub input_tokens: usize,
    pub output_tokens: usize,
}

/// A message in the conversation.
#[derive(Debug, Clone)]
pub struct LlmMessage {
    pub role: String,
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub tool_call_id: Option<String>,
}

impl LlmMessage {
    /// Create a user message.
    pub fn user(text: String) -> Self {
        Self {
            role: "user".into(),
            text,
            tool_calls: vec![],
            tool_call_id: None,
        }
    }
    /// Create an assistant message (text only).
    pub fn assistant(text: String) -> Self {
        Self {
            role: "assistant".into(),
            text,
            tool_calls: vec![],
            tool_call_id: None,
        }
    }
    /// Create an assistant message from a response.
    pub fn assistant_from(resp: &LlmResponse) -> Self {
        Self {
            role: "assistant".into(),
            text: resp.text.clone(),
            tool_calls: resp.tool_calls.clone(),
            tool_call_id: None,
        }
    }
    /// Create a tool-result message.
    pub fn tool_result(call_id: &str, content: &str) -> Self {
        Self {
            role: "tool".into(),
            text: content.into(),
            tool_calls: vec![],
            tool_call_id: Some(call_id.into()),
        }
    }
}

/// Narrow provider trait for LLM completions.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate a single completion.
    async fn complete(
        &self,
        messages: Vec<LlmMessage>,
        tools: Vec<ToolDefinition>,
        model: &str,
        temperature: Option<f32>,
    ) -> Result<LlmResponse>;
}

/// Trait for FunctionGemma-powered tool call reformatting.
#[async_trait]
pub trait ToolCallRewriter: Send + Sync {
    /// Extract structured tool calls from a text-only response.
    async fn maybe_reformat(
        &self,
        response: LlmResponse,
        tools: &[ToolDefinition],
        strict: bool,
    ) -> Result<LlmResponse>;
}

/// Trait for emitting RLM events (progress + completion).
pub trait RlmEventBus: Send + Sync {
    /// Emit a progress tick.
    fn emit_progress(&self, event: crate::RlmProgressEvent);
    /// Emit the terminal completion record.
    fn emit_completion(&self, event: crate::RlmCompletion);
}
