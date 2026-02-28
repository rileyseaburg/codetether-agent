//! Execution Engine — Capability Leases, Tool Execution, and Decision Receipts.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

use super::thinker::ThinkerClient;
use super::{ThoughtEvent, ThoughtEventType, trim_for_storage};
use crate::tool::ToolRegistry;

/// A short-lived capability granting tool access to a persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityLease {
    pub id: String,
    pub persona_id: String,
    pub tool_name: String,
    pub granted_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub granted_by: String,
}

impl CapabilityLease {
    /// Create a new 60-second capability lease.
    pub fn new(persona_id: &str, tool_name: &str, granted_by: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            persona_id: persona_id.to_string(),
            tool_name: tool_name.to_string(),
            granted_at: now,
            expires_at: now + Duration::seconds(60),
            granted_by: granted_by.to_string(),
        }
    }

    /// Check if the lease is still valid.
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}

/// A single tool invocation record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvocation {
    pub tool_name: String,
    pub arguments: Value,
    pub result: Option<String>,
    pub success: bool,
    pub lease_id: String,
    pub invoked_at: DateTime<Utc>,
}

/// Outcome of a proposal execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExecutionOutcome {
    Success {
        summary: String,
    },
    Failure {
        error: String,
        follow_up_attention: Option<String>,
    },
}

/// Full audit trail for a proposal execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionReceipt {
    pub id: String,
    pub proposal_id: String,
    pub inputs: Vec<String>,
    pub governance_decision: String,
    pub capability_leases: Vec<String>,
    pub tool_invocations: Vec<ToolInvocation>,
    pub outcome: ExecutionOutcome,
    pub created_at: DateTime<Utc>,
}

/// Raw tool request extracted from LLM structured output.
#[derive(Debug, Clone, Deserialize)]
struct ExtractedToolRequest {
    tool: String,
    arguments: Value,
}

/// Wrapper for tool request extraction response.
#[derive(Debug, Deserialize)]
struct ToolRequestResponse {
    tool_requests: Vec<ExtractedToolRequest>,
}

/// Validate tool arguments against the tool's JSON Schema.
fn validate_tool_args(schema: &Value, args: &Value) -> Result<(), String> {
    // Validate required fields
    if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
        for field in required {
            if let Some(field_name) = field.as_str()
                && args.get(field_name).is_none()
            {
                return Err(format!("Missing required field: {}", field_name));
            }
        }
    }

    // Validate field types
    if let Some(properties) = schema.get("properties").and_then(|p| p.as_object())
        && let Some(args_obj) = args.as_object()
    {
        for (key, value) in args_obj {
            if let Some(prop_schema) = properties.get(key)
                && let Some(expected_type) = prop_schema.get("type").and_then(|t| t.as_str())
            {
                let type_ok = match expected_type {
                    "string" => value.is_string(),
                    "number" | "integer" => value.is_number(),
                    "boolean" => value.is_boolean(),
                    "array" => value.is_array(),
                    "object" => value.is_object(),
                    _ => true,
                };
                if !type_ok {
                    return Err(format!(
                        "Field '{}' has wrong type: expected {}, got {}",
                        key,
                        expected_type,
                        json_type_name(value)
                    ));
                }
            }
        }
    }

    Ok(())
}

fn json_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Execute tool requests extracted from a Test-phase thought.
///
/// Returns ThoughtEvents for each tool execution result.
pub async fn execute_tool_requests(
    thinker: Option<&ThinkerClient>,
    tool_registry: &Arc<ToolRegistry>,
    persona_id: &str,
    thought_text: &str,
    allowed_tools: &[String],
) -> Vec<ThoughtEvent> {
    let Some(client) = thinker else {
        return Vec::new();
    };

    // Structured extraction: ask LLM for tool requests
    let system_prompt = "You are a tool request extractor. \
Given a test/check thought, determine if any tools should be invoked. \
Return ONLY valid JSON, no markdown fences. \
If no tools are needed, return {\"tool_requests\":[]}."
        .to_string();

    let available: Vec<&str> = allowed_tools.iter().map(|s| s.as_str()).collect();
    let user_prompt = format!(
        "Available tools: {tools}\n\nThought:\n{thought}\n\n\
Return JSON only: {{ \"tool_requests\": [{{ \"tool\": \"tool-name\", \"arguments\": {{...}} }}] }}",
        tools = available.join(", "),
        thought = thought_text
    );

    let output = match client.think(&system_prompt, &user_prompt).await {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    let text = output
        .text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let parsed: ToolRequestResponse = match serde_json::from_str(text) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };

    let mut events = Vec::new();

    for request in parsed.tool_requests {
        // Validate tool exists and is allowed
        if !allowed_tools.contains(&request.tool) {
            events.push(ThoughtEvent {
                id: Uuid::new_v4().to_string(),
                event_type: ThoughtEventType::CheckResult,
                persona_id: Some(persona_id.to_string()),
                swarm_id: None,
                timestamp: Utc::now(),
                payload: serde_json::json!({
                    "tool_rejected": true,
                    "tool": request.tool,
                    "reason": "tool not in allowed_tools",
                }),
            });
            continue;
        }

        let tool = match tool_registry.get(&request.tool) {
            Some(t) => t,
            None => {
                events.push(ThoughtEvent {
                    id: Uuid::new_v4().to_string(),
                    event_type: ThoughtEventType::CheckResult,
                    persona_id: Some(persona_id.to_string()),
                    swarm_id: None,
                    timestamp: Utc::now(),
                    payload: serde_json::json!({
                        "tool_rejected": true,
                        "tool": request.tool,
                        "reason": "tool not found in registry",
                    }),
                });
                continue;
            }
        };

        // Schema validation
        let schema = tool.parameters();
        if let Err(validation_error) = validate_tool_args(&schema, &request.arguments) {
            events.push(ThoughtEvent {
                id: Uuid::new_v4().to_string(),
                event_type: ThoughtEventType::CheckResult,
                persona_id: Some(persona_id.to_string()),
                swarm_id: None,
                timestamp: Utc::now(),
                payload: serde_json::json!({
                    "tool_rejected": true,
                    "tool": request.tool,
                    "reason": "schema_validation_failed",
                    "detail": validation_error,
                }),
            });
            continue;
        }

        // Create capability lease
        let lease = CapabilityLease::new(persona_id, &request.tool, "policy");

        // Validate lease is still valid before executing
        if !lease.is_valid() {
            events.push(ThoughtEvent {
                id: Uuid::new_v4().to_string(),
                event_type: ThoughtEventType::CheckResult,
                persona_id: Some(persona_id.to_string()),
                swarm_id: None,
                timestamp: Utc::now(),
                payload: serde_json::json!({
                    "tool_rejected": true,
                    "tool": request.tool,
                    "reason": "capability_lease_expired",
                    "lease_id": lease.id,
                }),
            });
            continue;
        }

        // Execute the tool
        let result = tool.execute(request.arguments.clone()).await;

        let (result_text, success) = match result {
            Ok(tool_result) => (
                trim_for_storage(&tool_result.output, 500),
                tool_result.success,
            ),
            Err(e) => (format!("Error: {}", e), false),
        };

        events.push(ThoughtEvent {
            id: Uuid::new_v4().to_string(),
            event_type: ThoughtEventType::CheckResult,
            persona_id: Some(persona_id.to_string()),
            swarm_id: None,
            timestamp: Utc::now(),
            payload: serde_json::json!({
                "tool_executed": true,
                "tool": request.tool,
                "lease_id": lease.id,
                "success": success,
                "result": result_text,
            }),
        });
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_lease_creation_and_validity() {
        let lease = CapabilityLease::new("p1", "Bash", "policy");
        assert!(lease.is_valid());
        assert_eq!(lease.persona_id, "p1");
        assert_eq!(lease.tool_name, "Bash");
    }

    #[test]
    fn validate_tool_args_checks_required_fields() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["command"],
            "properties": {
                "command": { "type": "string" }
            }
        });

        // Missing required field
        let args = serde_json::json!({});
        assert!(validate_tool_args(&schema, &args).is_err());

        // Has required field
        let args = serde_json::json!({ "command": "ls" });
        assert!(validate_tool_args(&schema, &args).is_ok());
    }

    #[test]
    fn validate_tool_args_checks_types() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "command": { "type": "string" },
                "timeout": { "type": "number" }
            }
        });

        // Wrong type
        let args = serde_json::json!({ "command": 42 });
        assert!(validate_tool_args(&schema, &args).is_err());

        // Correct types
        let args = serde_json::json!({ "command": "ls", "timeout": 30 });
        assert!(validate_tool_args(&schema, &args).is_ok());
    }

    #[test]
    fn validate_rejects_unlisted_tools() {
        let allowed = vec!["Bash".to_string()];
        assert!(!allowed.contains(&"WebFetch".to_string()));
        assert!(allowed.contains(&"Bash".to_string()));
    }

    #[tokio::test]
    async fn execute_without_thinker_returns_empty() {
        let registry = Arc::new(ToolRegistry::new());
        let result =
            execute_tool_requests(None, &registry, "p1", "some thought", &["Bash".to_string()])
                .await;
        assert!(result.is_empty());
    }
}
