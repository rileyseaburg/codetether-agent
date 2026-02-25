//! Shared relay context built from structured RLM deltas.

use crate::bus::{BusHandle, BusMessage};
use crate::provider::Provider;
use crate::rlm::{FinalPayload, RlmExecutor};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::sync::Arc;

const CONTEXT_KEY_PREFIX: &str = "relay.";
const CONTEXT_BUCKET_LIMIT: usize = 12;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextDelta {
    #[serde(default)]
    pub facts: Vec<String>,
    #[serde(default)]
    pub decisions: Vec<String>,
    #[serde(default)]
    pub risks: Vec<String>,
    #[serde(default)]
    pub next_actions: Vec<String>,
    #[serde(default)]
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SharedRelayContext {
    #[serde(default)]
    pub facts: Vec<String>,
    #[serde(default)]
    pub decisions: Vec<String>,
    #[serde(default)]
    pub risks: Vec<String>,
    #[serde(default)]
    pub next_actions: Vec<String>,
    #[serde(default)]
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDeltaEnvelope {
    pub relay_id: String,
    pub from_agent: String,
    pub round: usize,
    pub turn: usize,
    pub delta: ContextDelta,
}

impl SharedRelayContext {
    pub fn merge_delta(&mut self, delta: &ContextDelta) {
        merge_bucket(&mut self.facts, &delta.facts);
        merge_bucket(&mut self.decisions, &delta.decisions);
        merge_bucket(&mut self.risks, &delta.risks);
        merge_bucket(&mut self.next_actions, &delta.next_actions);
        merge_bucket(&mut self.evidence, &delta.evidence);
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
            && self.decisions.is_empty()
            && self.risks.is_empty()
            && self.next_actions.is_empty()
            && self.evidence.is_empty()
    }

    pub fn item_count(&self) -> usize {
        self.facts.len()
            + self.decisions.len()
            + self.risks.len()
            + self.next_actions.len()
            + self.evidence.len()
    }

    pub fn render_prompt_snapshot(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut lines = Vec::new();
        push_section(&mut lines, "Facts", &self.facts);
        push_section(&mut lines, "Decisions", &self.decisions);
        push_section(&mut lines, "Risks", &self.risks);
        push_section(&mut lines, "Next Actions", &self.next_actions);
        push_section(&mut lines, "Evidence", &self.evidence);
        lines.join("\n")
    }
}

pub fn context_result_key(relay_id: &str, turn: usize) -> String {
    format!("{CONTEXT_KEY_PREFIX}{relay_id}.context.{turn:04}")
}

pub fn context_result_prefix(relay_id: &str) -> String {
    format!("{CONTEXT_KEY_PREFIX}{relay_id}.context.")
}

pub fn publish_context_delta(
    handle: &BusHandle,
    relay_id: &str,
    from_agent: &str,
    round: usize,
    turn: usize,
    delta: &ContextDelta,
) -> usize {
    let envelope = ContextDeltaEnvelope {
        relay_id: relay_id.to_string(),
        from_agent: from_agent.to_string(),
        round,
        turn,
        delta: delta.clone(),
    };

    let value = serde_json::to_value(&envelope).unwrap_or(serde_json::json!({}));
    let tags = vec![
        "relay-context".to_string(),
        format!("relay:{relay_id}"),
        format!("agent:{from_agent}"),
    ];
    handle.publish_shared_result(context_result_key(relay_id, turn), value, tags)
}

pub fn drain_context_updates(
    receiver: &mut BusHandle,
    relay_id: &str,
    shared: &mut SharedRelayContext,
) -> usize {
    let topic_prefix = format!("results.{}", context_result_prefix(relay_id));
    let key_prefix = context_result_prefix(relay_id);
    let mut merged = 0usize;

    while let Some(envelope) = receiver.try_recv() {
        if !envelope.topic.starts_with(&topic_prefix) {
            continue;
        }

        let BusMessage::SharedResult { key, value, .. } = envelope.message else {
            continue;
        };
        if !key.starts_with(&key_prefix) {
            continue;
        }

        if let Ok(payload) = serde_json::from_value::<ContextDeltaEnvelope>(value.clone()) {
            if payload.relay_id == relay_id {
                shared.merge_delta(&payload.delta);
                merged += 1;
            }
            continue;
        }

        if let Ok(delta) = serde_json::from_value::<ContextDelta>(value) {
            shared.merge_delta(&delta);
            merged += 1;
        }
    }

    merged
}

pub fn compose_prompt_with_context(handoff: &str, shared: &SharedRelayContext) -> String {
    let snapshot = shared.render_prompt_snapshot();
    if snapshot.is_empty() {
        return handoff.to_string();
    }

    format!(
        "{handoff}\n\nShared Relay Context (cross-agent memory):\n{snapshot}\n\n\
Use this shared context as authoritative state across the relay."
    )
}

pub async fn distill_context_delta_with_rlm(
    output: &str,
    task: &str,
    from_agent: &str,
    provider_and_model: Option<(Arc<dyn Provider>, String)>,
) -> (ContextDelta, bool) {
    if let Some((provider, model_name)) = provider_and_model {
        let query = context_distillation_query(task, from_agent);
        let mut executor =
            RlmExecutor::new(output.to_string(), provider, model_name).with_max_iterations(2);
        match executor.analyze(&query).await {
            Ok(result) => {
                if let Some(delta) = extract_context_delta_from_text(&result.answer) {
                    return (delta, true);
                }
            }
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    agent = %from_agent,
                    "RLM context distillation failed; using fallback delta"
                );
            }
        }
    }

    (fallback_context_delta(output, from_agent), false)
}

pub fn parse_json_payload<T: DeserializeOwned>(text: &str) -> Option<T> {
    let trimmed = text.trim();
    if let Ok(value) = serde_json::from_str::<T>(trimmed) {
        return Some(value);
    }

    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}'))
        && start < end
        && let Ok(value) = serde_json::from_str::<T>(&trimmed[start..=end])
    {
        return Some(value);
    }

    None
}

fn context_distillation_query(task: &str, from_agent: &str) -> String {
    format!(
        "Task:\n{task}\n\nAgent turn source: @{from_agent}\n\n\
Distill this turn into shared relay context.\n\
Return FINAL(JSON) with this shape:\n\
{{\"kind\":\"semantic\",\"file\":\"relay_context_delta\",\"answer\":\"{{\\\"facts\\\":[\\\"...\\\"],\\\"decisions\\\":[\\\"...\\\"],\\\"risks\\\":[\\\"...\\\"],\\\"next_actions\\\":[\\\"...\\\"],\\\"evidence\\\":[\\\"...\\\"]}}\"}}\n\
Rules:\n\
- answer MUST be valid JSON object encoded as a string\n\
- 0-3 items per field\n\
- no markdown"
    )
}

fn extract_context_delta_from_text(text: &str) -> Option<ContextDelta> {
    if let Some(delta) = parse_json_payload::<ContextDelta>(text) {
        return Some(delta);
    }

    if let FinalPayload::Semantic(payload) = FinalPayload::parse(text) {
        if let Some(delta) = parse_json_payload::<ContextDelta>(&payload.answer) {
            return Some(delta);
        }
    }

    None
}

fn fallback_context_delta(output: &str, from_agent: &str) -> ContextDelta {
    let excerpt = truncate_chars(output.trim(), 240);
    let fact = if excerpt.is_empty() {
        format!("@{from_agent} produced an empty turn output")
    } else {
        format!("@{from_agent}: {excerpt}")
    };

    ContextDelta {
        facts: vec![fact],
        decisions: Vec::new(),
        risks: Vec::new(),
        next_actions: vec!["Continue with one concrete implementation step.".to_string()],
        evidence: vec![format!("output_chars={}", output.chars().count())],
    }
}

fn merge_bucket(target: &mut Vec<String>, additions: &[String]) {
    for item in additions {
        let normalized = item.trim();
        if normalized.is_empty() {
            continue;
        }
        if target.iter().any(|existing| existing == normalized) {
            continue;
        }
        target.push(normalized.to_string());
        if target.len() > CONTEXT_BUCKET_LIMIT {
            target.remove(0);
        }
    }
}

fn push_section(lines: &mut Vec<String>, title: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }
    lines.push(format!("{title}:"));
    lines.extend(values.iter().map(|v| format!("- {v}")));
}

fn truncate_chars(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.to_string();
    }
    input.chars().take(max_chars).collect::<String>() + "..."
}

#[cfg(test)]
mod tests {
    use super::{ContextDelta, SharedRelayContext, parse_json_payload};

    #[test]
    fn parse_json_payload_extracts_embedded_object() {
        let parsed = parse_json_payload::<ContextDelta>(
            "noise {\"facts\":[\"a\"],\"decisions\":[],\"risks\":[],\"next_actions\":[],\"evidence\":[]} trailing",
        );
        assert!(parsed.is_some());
    }

    #[test]
    fn merge_delta_deduplicates_items() {
        let mut shared = SharedRelayContext::default();
        let delta = ContextDelta {
            facts: vec!["same".to_string(), "same".to_string()],
            decisions: Vec::new(),
            risks: Vec::new(),
            next_actions: Vec::new(),
            evidence: Vec::new(),
        };
        shared.merge_delta(&delta);
        assert_eq!(shared.facts.len(), 1);
    }

    #[test]
    fn render_prompt_snapshot_has_sections() {
        let mut shared = SharedRelayContext::default();
        shared.facts.push("f1".to_string());
        let rendered = shared.render_prompt_snapshot();
        assert!(rendered.contains("Facts:"));
        assert!(rendered.contains("- f1"));
    }
}
