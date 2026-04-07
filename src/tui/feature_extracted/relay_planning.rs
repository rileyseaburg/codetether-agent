//! Relay planning: agent profile generation, OKR drafting, and spawn decisions
//!
//! Structures and functions for planning multi-agent relay composition.

use anyhow::Result;
use serde::{Deserialize, de::DeserializeOwned};
use std::collections::HashMap;
use uuid::Uuid;

use crate::okr::{KeyResult, Okr};


struct PlannedRelayProfile {
    #[serde(default)]
    name: String,
    #[serde(default)]
    specialty: String,
    #[serde(default)]
    mission: String,
    #[serde(default)]
    capabilities: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct PlannedRelayResponse {
    #[serde(default)]
    profiles: Vec<PlannedRelayProfile>,
}

#[derive(Debug, Clone, Deserialize)]
struct RelaySpawnDecision {
    #[serde(default)]
    spawn: bool,
    #[serde(default)]
    reason: String,
    #[serde(default)]
    profile: Option<PlannedRelayProfile>,
}

#[derive(Debug, Clone, Deserialize)]
struct PlannedOkrKeyResult {
    #[serde(default)]
    title: String,
    #[serde(default)]
    target_value: f64,
    #[serde(default = "default_okr_unit")]
    unit: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PlannedOkrDraft {
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    key_results: Vec<PlannedOkrKeyResult>,
}


fn default_okr_unit() -> String {
    "%".to_string()
}


fn slugify_label(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_dash = false;

    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}


fn sanitize_relay_agent_name(value: &str) -> String {
    let raw = slugify_label(value);
    let base = if raw.is_empty() {
        "auto-specialist".to_string()
    } else if raw.starts_with("auto-") {
        raw
    } else {
        format!("auto-{raw}")
    };

    truncate_with_ellipsis(&base, 48)
        .trim_end_matches("...")
        .to_string()
}


fn sanitize_spawned_agent_name(value: &str) -> String {
    let slug = slugify_label(value);
    let bounded = truncate_with_ellipsis(&slug, 48);
    bounded.trim_end_matches("...").to_string()
}


fn unique_relay_agent_name(base: &str, existing: &[String]) -> String {
    if !existing.iter().any(|name| name == base) {
        return base.to_string();
    }

    let mut suffix = 2usize;
    loop {
        let candidate = format!("{base}-{suffix}");
        if !existing.iter().any(|name| name == &candidate) {
            return candidate;
        }
        suffix += 1;
    }
}


fn relay_instruction_from_plan(name: &str, specialty: &str, mission: &str) -> String {
    format!(
        "You are @{name}.\n\
         Specialty: {specialty}.\n\
         Mission: {mission}\n\n\
         This is a protocol-first relay conversation. Treat incoming handoffs as authoritative context.\n\
         Keep responses concise, concrete, and useful for the next specialist.\n\
         Include one clear recommendation for what the next agent should do.\n\
         If the task is too large for the current team, explicitly call out missing specialties and handoff boundaries.",
    )
}


fn build_runtime_profile_from_plan(
    profile: PlannedRelayProfile,
    existing: &[String],
) -> Option<(String, String, Vec<String>)> {
    let specialty = if profile.specialty.trim().is_empty() {
        "generalist".to_string()
    } else {
        profile.specialty.trim().to_string()
    };

    let mission = if profile.mission.trim().is_empty() {
        "Advance the relay with concrete next actions and clear handoffs.".to_string()
    } else {
        profile.mission.trim().to_string()
    };

    let base_name = if profile.name.trim().is_empty() {
        format!("auto-{}", slugify_label(&specialty))
    } else {
        profile.name.trim().to_string()
    };

    let sanitized = sanitize_relay_agent_name(&base_name);
    let name = unique_relay_agent_name(&sanitized, existing);
    if name.trim().is_empty() {
        return None;
    }

    let mut capabilities: Vec<String> = Vec::new();
    let specialty_cap = slugify_label(&specialty);
    if !specialty_cap.is_empty() {
        capabilities.push(specialty_cap);
    }

    for capability in profile.capabilities {
        let normalized = slugify_label(&capability);
        if !normalized.is_empty() && !capabilities.contains(&normalized) {
            capabilities.push(normalized);
        }
    }

    crate::autochat::ensure_required_relay_capabilities(&mut capabilities);

    let instructions = relay_instruction_from_plan(&name, &specialty, &mission);
    Some((name, instructions, capabilities))
}


fn extract_json_payload<T: DeserializeOwned>(text: &str) -> Option<T> {
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

    if let (Some(start), Some(end)) = (trimmed.find('['), trimmed.rfind(']'))
        && start < end
        && let Ok(value) = serde_json::from_str::<T>(&trimmed[start..=end])
    {
        return Some(value);
    }

    None
}


fn default_relay_okr_template(okr_id: Uuid, task: &str) -> Okr {
    let mut okr = Okr::new(
        format!("Relay: {}", truncate_with_ellipsis(task, 60)),
        format!("Execute relay task: {}", task),
    );
    okr.id = okr_id;

    okr.add_key_result(KeyResult::new(
        okr_id,
        "Relay completes all rounds",
        100.0,
        "%",
    ));
    okr.add_key_result(KeyResult::new(
        okr_id,
        "Team produces actionable handoff",
        1.0,
        "count",
    ));
    okr.add_key_result(KeyResult::new(okr_id, "No critical errors", 0.0, "count"));

    okr
}


fn okr_from_planned_draft(okr_id: Uuid, task: &str, planned: PlannedOkrDraft) -> Okr {
    let title = if planned.title.trim().is_empty() {
        format!("Relay: {}", truncate_with_ellipsis(task, 60))
    } else {
        planned.title.trim().to_string()
    };

    let description = if planned.description.trim().is_empty() {
        format!("Execute relay task: {}", task)
    } else {
        planned.description.trim().to_string()
    };

    let mut okr = Okr::new(title, description);
    okr.id = okr_id;

    for kr in planned.key_results.into_iter().take(7) {
        if kr.title.trim().is_empty() {
            continue;
        }

        let unit = if kr.unit.trim().is_empty() {
            default_okr_unit()
        } else {
            kr.unit
        };

        okr.add_key_result(KeyResult::new(
            okr_id,
            kr.title.trim().to_string(),
            kr.target_value.max(0.0),
            unit,
        ));
    }

    if okr.key_results.is_empty() {
        default_relay_okr_template(okr_id, task)
    } else {
        okr
    }
}

async fn plan_okr_draft_with_registry(
    task: &str,
    model_ref: &str,
    agent_count: usize,
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
) -> Option<PlannedOkrDraft> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry, model_ref)?;
    let model_name_for_log = model_name.clone();

    let request = crate::provider::CompletionRequest {
        model: model_name,
        messages: vec![
            crate::provider::Message {
                role: crate::provider::Role::System,
                content: vec![crate::provider::ContentPart::Text {
                    text: "You write OKRs for execution governance. Return ONLY valid JSON."
                        .to_string(),
                }],
            },
            crate::provider::Message {
                role: crate::provider::Role::User,
                content: vec![crate::provider::ContentPart::Text {
                    text: format!(
                        "Task:\n{task}\n\nTeam size: {agent_count}\n\n\
                         Propose ONE objective and 3-7 measurable key results for executing this task via an AI relay.\n\
                         Key results must be quantitative (numeric target_value + unit).\n\n\
                         Return JSON ONLY (no markdown):\n\
                         {{\n  \"title\": \"...\",\n  \"description\": \"...\",\n  \"key_results\": [\n    {{\"title\":\"...\",\"target_value\":123,\"unit\":\"%|count|tests|files|items\"}}\n  ]\n}}\n\n\
                         Rules:\n\
                         - Avoid vague KRs like 'do better'\n\
                         - Prefer engineering outcomes (tests passing, endpoints implemented, docs updated, errors=0)\n\
                         - If unsure about a unit, use 'count'"
                    ),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(0.4),
        top_p: Some(0.9),
        max_tokens: Some(900),
        stop: Vec::new(),
    };

    let response = provider.complete(request).await.ok()?;
    let text = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            crate::provider::ContentPart::Text { text }
            | crate::provider::ContentPart::Thinking { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    tracing::debug!(
        model = %model_name_for_log,
        response_len = text.len(),
        response_preview = %text.chars().take(500).collect::<String>(),
        "OKR draft model response"
    );

    let parsed = extract_json_payload::<PlannedOkrDraft>(&text);
    if parsed.is_none() {
        tracing::warn!(
            model = %model_name_for_log,
            response_preview = %text.chars().take(500).collect::<String>(),
            "Failed to parse OKR draft JSON from model response"
        );
    }
    parsed
}


async fn plan_relay_profiles_with_registry(
    task: &str,
    model_ref: &str,
    requested_agents: usize,
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
) -> Option<Vec<(String, String, Vec<String>)>> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry, model_ref)?;
    let requested_agents = requested_agents.clamp(2, AUTOCHAT_MAX_AGENTS);

    let request = crate::provider::CompletionRequest {
        model: model_name,
        messages: vec![
            crate::provider::Message {
                role: crate::provider::Role::System,
                content: vec![crate::provider::ContentPart::Text {
                    text: "You are a relay-team architect. Return ONLY valid JSON.".to_string(),
                }],
            },
            crate::provider::Message {
                role: crate::provider::Role::User,
                content: vec![crate::provider::ContentPart::Text {
                    text: format!(
                        "Task:\n{task}\n\nDesign a task-specific relay team.\n\
                         Respond with JSON object only:\n\
                         {{\n  \"profiles\": [\n    {{\"name\":\"auto-...\",\"specialty\":\"...\",\"mission\":\"...\",\"capabilities\":[\"...\"]}}\n  ]\n}}\n\
                         Requirements:\n\
                         - Return {} profiles\n\
                         - Names must be short kebab-case\n\
                         - Capabilities must be concise skill tags\n\
                         - Missions should be concrete and handoff-friendly",
                        requested_agents
                    ),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(1.0),
        top_p: Some(0.9),
        max_tokens: Some(1200),
        stop: Vec::new(),
    };

    let response = provider.complete(request).await.ok()?;
    let text = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            crate::provider::ContentPart::Text { text }
            | crate::provider::ContentPart::Thinking { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    let planned = extract_json_payload::<PlannedRelayResponse>(&text)?;
    let mut existing = Vec::<String>::new();
    let mut runtime = Vec::<(String, String, Vec<String>)>::new();

    for profile in planned.profiles.into_iter().take(AUTOCHAT_MAX_AGENTS) {
        if let Some((name, instructions, capabilities)) =
            build_runtime_profile_from_plan(profile, &existing)
        {
            existing.push(name.clone());
            runtime.push((name, instructions, capabilities));
        }
    }

    if runtime.len() >= 2 {
        Some(runtime)
    } else {
        None
    }
}


async fn decide_dynamic_spawn_with_registry(
    task: &str,
    model_ref: &str,
    latest_output: &str,
    round: usize,
    ordered_agents: &[String],
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
) -> Option<(String, String, Vec<String>, String)> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry, model_ref)?;
    let team = ordered_agents
        .iter()
        .map(|name| format!("@{name}"))
        .collect::<Vec<_>>()
        .join(", ");
    let output_excerpt = truncate_with_ellipsis(latest_output, 2200);

    let request = crate::provider::CompletionRequest {
        model: model_name,
        messages: vec![
            crate::provider::Message {
                role: crate::provider::Role::System,
                content: vec![crate::provider::ContentPart::Text {
                    text: "You are a relay scaling controller. Return ONLY valid JSON.".to_string(),
                }],
            },
            crate::provider::Message {
                role: crate::provider::Role::User,
                content: vec![crate::provider::ContentPart::Text {
                    text: format!(
                        "Task:\n{task}\n\nRound: {round}\nCurrent team: {team}\n\
                         Latest handoff excerpt:\n{output_excerpt}\n\n\
                         Decide whether the team needs one additional specialist right now.\n\
                         Respond with JSON object only:\n\
                         {{\n  \"spawn\": true|false,\n  \"reason\": \"...\",\n  \"profile\": {{\"name\":\"auto-...\",\"specialty\":\"...\",\"mission\":\"...\",\"capabilities\":[\"...\"]}}\n}}\n\
                         If spawn=false, profile may be null or omitted."
                    ),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(1.0),
        top_p: Some(0.9),
        max_tokens: Some(420),
        stop: Vec::new(),
    };

    let response = provider.complete(request).await.ok()?;
    let text = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            crate::provider::ContentPart::Text { text }
            | crate::provider::ContentPart::Thinking { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    let decision = extract_json_payload::<RelaySpawnDecision>(&text)?;
    if !decision.spawn {
        return None;
    }

    let profile = decision.profile.unwrap();
    let (name, instructions, capabilities) =
        build_runtime_profile_from_plan(profile, ordered_agents)?;
    let reason = if decision.reason.trim().is_empty() {
        "Model requested additional specialist for task scope.".to_string()
    } else {
        decision.reason.trim().to_string()
    };

    Some((name, instructions, capabilities, reason))
}


async fn prepare_autochat_handoff_with_registry(
    task: &str,
    from_agent: &str,
    output: &str,
    model_ref: &str,
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
) -> (String, bool) {
    let mut used_rlm = false;
    let mut relay_payload = if output.len() > AUTOCHAT_RLM_THRESHOLD_CHARS {
        truncate_with_ellipsis(output, AUTOCHAT_RLM_FALLBACK_CHARS)
    } else {
        output.to_string()
    };

    if let Some((provider, model_name)) = resolve_provider_for_model_autochat(registry, model_ref) {
        let mut executor =
            RlmExecutor::new(output.to_string(), provider, model_name).with_max_iterations(2);
        match executor.analyze(AUTOCHAT_RLM_HANDOFF_QUERY).await {
            Ok(result) => {
                let normalized = extract_semantic_handoff_from_rlm(&result.answer);
                if !normalized.is_empty() {
                    relay_payload = normalized;
                    used_rlm = true;
                }
            }
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "RLM handoff normalization failed; using fallback payload"
                );
            }
        }
    } else {
        tracing::warn!(
            model_ref = %model_ref,
            "No provider resolved for RLM handoff normalization; using fallback payload"
        );
    }

    (
        format!(
            "Relay task:\n{task}\n\nIncoming handoff from @{from_agent}:\n{relay_payload}\n\n\
             Continue the work from this handoff. Keep your response focused and provide one concrete next-step instruction for the next agent."
        ),
        used_rlm,
    )
}

