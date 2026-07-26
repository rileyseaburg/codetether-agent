//! Tool-schema catalog announcements for training-data fidelity.
//!
//! Inference-time chat templates inject an available-tools block into the
//! system prompt. Historically the bus only recorded capability *names*
//! ([`BusMessage::AgentReady`](super::BusMessage::AgentReady)), so exported
//! training conversations never contained that block. A model fine-tuned on
//! those transcripts then saw the tools block for the first time at inference
//! and echoed it back instead of emitting a tool call.
//!
//! This module records the JSON schemas an agent actually had available so
//! exported transcripts can reproduce the inference-time prompt exactly.

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::{AgentBus, BusEnvelope, BusMessage};

/// One tool's name, description, and JSON-Schema parameters.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::bus::tool_catalog::ToolSchema;
///
/// let schema = ToolSchema {
///     name: "read".into(),
///     description: "Read a file".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// };
/// assert_eq!(schema.name, "read");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolSchema {
    /// Tool name as the model must emit it.
    pub name: String,
    /// Human-readable purpose shown to the model.
    pub description: String,
    /// JSON Schema for the tool arguments.
    pub parameters: serde_json::Value,
}

/// Render schemas as the OpenAI-style function entries used by chat templates.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::bus::tool_catalog::{ToolSchema, to_function_specs};
///
/// let specs = to_function_specs(&[ToolSchema {
///     name: "read".into(),
///     description: "Read a file".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// }]);
/// assert_eq!(specs[0]["type"], "function");
/// assert_eq!(specs[0]["function"]["name"], "read");
/// ```
pub fn to_function_specs(schemas: &[ToolSchema]) -> Vec<serde_json::Value> {
    schemas.iter().map(function_spec).collect()
}

fn function_spec(schema: &ToolSchema) -> serde_json::Value {
    serde_json::json!({
        "type": "function",
        "function": {
            "name": schema.name,
            "description": schema.description,
            "parameters": schema.parameters,
        }
    })
}

/// Convert live LLM tool definitions into catalog schemas.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::bus::tool_catalog::from_definitions;
/// use codetether_agent::provider::ToolDefinition;
///
/// let schemas = from_definitions(&[ToolDefinition {
///     name: "read".into(),
///     description: "Read a file".into(),
///     parameters: serde_json::json!({"type": "object"}),
/// }]);
/// assert_eq!(schemas[0].name, "read");
/// ```
pub fn from_definitions(definitions: &[crate::provider::ToolDefinition]) -> Vec<ToolSchema> {
    definitions
        .iter()
        .map(|d| ToolSchema {
            name: d.name.clone(),
            description: d.description.clone(),
            parameters: d.parameters.clone(),
        })
        .collect()
}

/// Build and publish one envelope on behalf of an agent handle.
///
/// Shared by [`BusHandle::send_with_correlation`](super::BusHandle) and the
/// catalog announcement so envelope construction lives in one place.
pub(super) fn publish(
    bus: &Arc<AgentBus>,
    agent_id: &str,
    topic: String,
    message: BusMessage,
    correlation_id: Option<String>,
) -> usize {
    bus.publish(BusEnvelope {
        id: uuid::Uuid::new_v4().to_string(),
        topic,
        sender_id: agent_id.to_string(),
        correlation_id,
        timestamp: chrono::Utc::now(),
        message,
    })
}
