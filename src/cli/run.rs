//! Non-interactive run command

use super::RunArgs;
use crate::bus::{AgentBus, relay::ProtocolRelayRuntime, relay::RelayAgentProfile};
use crate::config::Config;
use crate::okr::{ApprovalDecision, KeyResult, Okr, OkrRepository, OkrRun, OkrRunStatus};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use anyhow::Result;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::io::Write;
use uuid::Uuid;

const AUTOCHAT_MAX_AGENTS: usize = 100;
const AUTOCHAT_DEFAULT_AGENTS: usize = 3;
const AUTOCHAT_MAX_ROUNDS: usize = 3;
const AUTOCHAT_MAX_DYNAMIC_SPAWNS: usize = 3;
const AUTOCHAT_SPAWN_CHECK_MIN_CHARS: usize = 800;
const AUTOCHAT_QUICK_DEMO_TASK: &str = "Self-organize into the right specialties for this task, then relay one concrete implementation plan with clear next handoffs.";
const GO_DEFAULT_MODEL: &str = "minimax-credits/MiniMax-M2.5-highspeed";

/// Guarded UUID parse that logs warnings on invalid input instead of returning NIL UUID.
/// Returns None for invalid UUIDs, allowing callers to skip operations rather than corrupt data.
fn parse_uuid_guarded(s: &str, context: &str) -> Option<Uuid> {
    match s.parse::<Uuid>() {
        Ok(uuid) => Some(uuid),
        Err(e) => {
            tracing::warn!(
                context,
                uuid_str = %s,
                error = %e,
                "Invalid UUID string - skipping operation"
            );
            None
        }
    }
}

#[derive(Debug, Clone)]
struct RelayProfile {
    name: String,
    instructions: String,
    capabilities: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Serialize)]
struct AutochatCliResult {
    status: String,
    relay_id: String,
    model: String,
    agent_count: usize,
    turns: usize,
    agents: Vec<String>,
    final_handoff: String,
    summary: String,
    failure: Option<String>,
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
) -> Option<RelayProfile> {
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

    for required in ["relay", "context-handoff", "autochat"] {
        if !capabilities.iter().any(|capability| capability == required) {
            capabilities.push(required.to_string());
        }
    }

    Some(RelayProfile {
        name: name.clone(),
        instructions: relay_instruction_from_plan(&name, &specialty, &mission),
        capabilities,
    })
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

fn resolve_provider_for_model_autochat(
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
    model_ref: &str,
) -> Option<(std::sync::Arc<dyn crate::provider::Provider>, String)> {
    let (provider_name, model_name) = crate::provider::parse_model_string(model_ref);
    if let Some(provider_name) = provider_name
        && let Some(provider) = registry.get(provider_name)
    {
        return Some((provider, model_name.to_string()));
    }

    let fallbacks = [
        "zai",
        "openai",
        "github-copilot",
        "anthropic",
        "openrouter",
        "novita",
        "moonshotai",
        "google",
    ];

    for provider_name in fallbacks {
        if let Some(provider) = registry.get(provider_name) {
            return Some((provider, model_ref.to_string()));
        }
    }

    registry
        .list()
        .first()
        .copied()
        .and_then(|name| registry.get(name))
        .map(|provider| (provider, model_ref.to_string()))
}

async fn plan_relay_profiles_with_registry(
    task: &str,
    model_ref: &str,
    requested_agents: usize,
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
) -> Option<Vec<RelayProfile>> {
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
    let mut runtime = Vec::<RelayProfile>::new();

    for profile in planned.profiles.into_iter().take(AUTOCHAT_MAX_AGENTS) {
        if let Some(runtime_profile) = build_runtime_profile_from_plan(profile, &existing) {
            existing.push(runtime_profile.name.clone());
            runtime.push(runtime_profile);
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
) -> Option<(RelayProfile, String)> {
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

    let profile = decision.profile?;
    let runtime_profile = build_runtime_profile_from_plan(profile, ordered_agents)?;
    let reason = if decision.reason.trim().is_empty() {
        "Model requested additional specialist for task scope.".to_string()
    } else {
        decision.reason.trim().to_string()
    };

    Some((runtime_profile, reason))
}

pub async fn execute(args: RunArgs) -> Result<()> {
    let message = args.message.trim();

    if message.is_empty() {
        anyhow::bail!("You must provide a message");
    }

    tracing::info!("Running with message: {}", message);

    // Load configuration
    let config = Config::load().await.unwrap_or_default();

    // Protocol-first relay aliases in CLI:
    // - /go [count] <task>
    // - /autochat [count] <task>
    let easy_go_requested = is_easy_go_command(message);
    let normalized = normalize_cli_go_command(message);
    if let Some(rest) = command_with_optional_args(&normalized, "/autochat") {
        let Some((agent_count, task)) = parse_autochat_args(rest) else {
            anyhow::bail!(
                "Usage: /autochat [count] <task>\nEasy mode: /go <task>\ncount range: 2-{} (default: {})",
                AUTOCHAT_MAX_AGENTS,
                AUTOCHAT_DEFAULT_AGENTS
            );
        };

        if !(2..=AUTOCHAT_MAX_AGENTS).contains(&agent_count) {
            anyhow::bail!(
                "Invalid relay size {}. count must be between 2 and {}",
                agent_count,
                AUTOCHAT_MAX_AGENTS
            );
        }

        let model = resolve_autochat_model(
            args.model.as_deref(),
            std::env::var("CODETETHER_DEFAULT_MODEL").ok().as_deref(),
            config.default_model.as_deref(),
            easy_go_requested,
        );

        // For /go commands (not /autochat), require OKR approval then execute via Ralph
        if easy_go_requested {
            // For /go, default to max concurrency (run all stories in parallel)
            // unless the user explicitly specified a count like "/go 5 task"
            let max_concurrent = if rest.trim().starts_with(|c: char| c.is_ascii_digit()) {
                agent_count
            } else {
                AUTOCHAT_MAX_AGENTS
            };
            // Create OKR draft
            let okr_id = Uuid::new_v4();
            let mut okr = Okr::new(
                format!("Relay: {}", truncate_with_ellipsis(&task, 60)),
                format!("Execute relay task: {}", task),
            );
            okr.id = okr_id;

            // Add default key results
            let kr1 = KeyResult::new(okr_id, "Relay completes all rounds", 100.0, "%");
            let kr2 = KeyResult::new(okr_id, "Team produces actionable handoff", 1.0, "count");
            let kr3 = KeyResult::new(okr_id, "No critical errors", 0.0, "count");
            okr.add_key_result(kr1);
            okr.add_key_result(kr2);
            okr.add_key_result(kr3);

            // Create run
            let mut run = OkrRun::new(
                okr_id,
                format!("Run {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
            );
            let _ = run.submit_for_approval();

            // Show OKR draft
            println!("\n⚠️  /go OKR Draft\n");
            println!("Task: {}", truncate_with_ellipsis(&task, 80));
            println!("Agents: {} | Model: {}", agent_count, model);
            println!("\nObjective: {}", okr.title);
            println!("\nKey Results:");
            for kr in &okr.key_results {
                println!("  • {} (target: {} {})", kr.title, kr.target_value, kr.unit);
            }
            println!("\n");

            // Prompt for approval
            print!("Approve OKR and start relay? [y/n]: ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let input = input.trim().to_lowercase();
            if input != "y" && input != "yes" {
                run.record_decision(ApprovalDecision::deny(run.id, "User denied via CLI"));
                println!("❌ OKR denied. Relay not started.");
                println!("Use /autochat for tactical execution without OKR tracking.");
                return Ok(());
            }

            println!("✅ OKR approved! Starting Ralph PRD execution...\n");

            // Save OKR and run
            let mut approved_run = run;
            if let Ok(repo) = OkrRepository::from_config().await {
                let _ = repo.create_okr(okr.clone()).await;
                approved_run.record_decision(ApprovalDecision::approve(approved_run.id, "User approved via CLI"));
                approved_run.correlation_id = Some(format!("ralph-{}", Uuid::new_v4()));
                let _ = repo.create_run(approved_run.clone()).await;
                tracing::info!(okr_id = %okr_id, okr_run_id = %approved_run.id, "OKR run approved and saved");
            }

            // Load provider for Ralph execution
            let registry = std::sync::Arc::new(crate::provider::ProviderRegistry::from_vault().await?);
            let (provider, resolved_model) = resolve_provider_for_model_autochat(&registry, &model)
                .ok_or_else(|| anyhow::anyhow!("No provider available for model '{model}'"))?;

            // Wire bus for training data capture
            let bus = crate::bus::AgentBus::new().into_arc();
            crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

            // Execute via Ralph PRD loop — use max_concurrent as concurrency
            let ralph_result = super::go_ralph::execute_go_ralph(
                task,
                &mut okr,
                &mut approved_run,
                provider,
                &resolved_model,
                10, // max iterations
                Some(bus), // bus for training data
                max_concurrent, // max concurrent stories
                Some(registry.clone()), // relay registry
            )
            .await?;

            // Persist final run state
            if let Ok(repo) = OkrRepository::from_config().await {
                let _ = repo.update_run(approved_run).await;
            }

            // Display results
            match args.format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                    "passed": ralph_result.passed,
                    "total": ralph_result.total,
                    "all_passed": ralph_result.all_passed,
                    "iterations": ralph_result.iterations,
                    "feature_branch": ralph_result.feature_branch,
                    "prd_path": ralph_result.prd_path.display().to_string(),
                    "status": format!("{:?}", ralph_result.status),
                    "stories": ralph_result.stories.iter().map(|s| serde_json::json!({
                        "id": s.id,
                        "title": s.title,
                        "passed": s.passed,
                    })).collect::<Vec<_>>(),
                }))?),
                _ => {
                    println!(
                        "{}",
                        super::go_ralph::format_go_ralph_result(&ralph_result, task)
                    );
                }
            }
            return Ok(());
        }

        // Plain /autochat (no OKR) — use traditional relay
        let relay_result =
            run_protocol_first_relay(agent_count, task, &model, None, None).await?;
        match args.format.as_str() {
            "json" => println!("{}", serde_json::to_string_pretty(&relay_result)?),
            _ => {
                println!("{}", relay_result.summary);
                if let Some(failure) = &relay_result.failure {
                    eprintln!("\nFailure detail: {}", failure);
                }
                eprintln!(
                    "\n[Relay: {} | Model: {}]",
                    relay_result.relay_id, relay_result.model
                );
            }
        }
        return Ok(());
    }

    // Create or continue session.
    let mut session = if let Some(session_id) = args.session.clone() {
        tracing::info!("Continuing session: {}", session_id);
        if let Some(oc_id) = session_id.strip_prefix("opencode_") {
            if let Some(storage) = crate::opencode::OpenCodeStorage::new() {
                Session::from_opencode(oc_id, &storage).await?
            } else {
                anyhow::bail!("OpenCode storage not available")
            }
        } else {
            Session::load(&session_id).await?
        }
    } else if args.continue_session {
        let workspace_dir = std::env::current_dir().unwrap_or_default();
        match Session::last_for_directory(Some(&workspace_dir)).await {
            Ok(s) => {
                tracing::info!(
                    session_id = %s.id,
                    workspace = %workspace_dir.display(),
                    "Continuing last workspace session"
                );
                s
            }
            Err(_) => {
                // Fallback: try to resume from OpenCode session
                match Session::last_opencode_for_directory(&workspace_dir).await {
                    Ok(s) => {
                        tracing::info!(
                            session_id = %s.id,
                            workspace = %workspace_dir.display(),
                            "Resuming from OpenCode session"
                        );
                        s
                    }
                    Err(_) => {
                        let s = Session::new().await?;
                        tracing::info!(
                            session_id = %s.id,
                            workspace = %workspace_dir.display(),
                            "No workspace session found; created new session"
                        );
                        s
                    }
                }
            }
        }
    } else {
        let s = Session::new().await?;
        tracing::info!("Created new session: {}", s.id);
        s
    };

    // Set model: CLI arg > env var > config default
    let model = args
        .model
        .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
        .or(config.default_model);

    if let Some(model) = model {
        tracing::info!("Using model: {}", model);
        session.metadata.model = Some(model);
    }

    // Wire bus for thinking capture + S3 training data
    let bus = AgentBus::new().into_arc();
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
    session.bus = Some(bus);

    // Execute the prompt
    let result = session.prompt(message).await?;

    // Output based on format
    match args.format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        _ => {
            println!("{}", result.text);
            // Show session ID for continuation
            eprintln!(
                "\n[Session: {} | Continue with: codetether run -c \"...\"]",
                session.id
            );
        }
    }

    Ok(())
}

fn command_with_optional_args<'a>(input: &'a str, command: &str) -> Option<&'a str> {
    let trimmed = input.trim();
    let rest = trimmed.strip_prefix(command)?;

    if rest.is_empty() {
        return Some("");
    }

    let first = rest.chars().next()?;
    if first.is_whitespace() {
        Some(rest.trim())
    } else {
        None
    }
}

fn normalize_cli_go_command(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() || !trimmed.starts_with('/') {
        return trimmed.to_string();
    }

    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("").trim();

    match command.to_ascii_lowercase().as_str() {
        "/go" | "/team" => {
            if args.is_empty() {
                format!(
                    "/autochat {} {}",
                    AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_QUICK_DEMO_TASK
                )
            } else {
                let mut count_and_task = args.splitn(2, char::is_whitespace);
                let first = count_and_task.next().unwrap_or("");
                if let Ok(count) = first.parse::<usize>() {
                    let task = count_and_task.next().unwrap_or("").trim();
                    if task.is_empty() {
                        format!("/autochat {count} {AUTOCHAT_QUICK_DEMO_TASK}")
                    } else {
                        format!("/autochat {count} {task}")
                    }
                } else {
                    format!("/autochat {} {args}", AUTOCHAT_DEFAULT_AGENTS)
                }
            }
        }
        _ => trimmed.to_string(),
    }
}

fn is_easy_go_command(input: &str) -> bool {
    let command = input
        .trim_start()
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(command.as_str(), "/go" | "/team")
}

fn parse_autochat_args(rest: &str) -> Option<(usize, &str)> {
    let rest = rest.trim();
    if rest.is_empty() {
        return None;
    }

    let mut parts = rest.splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or("").trim();
    if first.is_empty() {
        return None;
    }

    if let Ok(count) = first.parse::<usize>() {
        let task = parts.next().unwrap_or("").trim();
        if task.is_empty() {
            Some((count, AUTOCHAT_QUICK_DEMO_TASK))
        } else {
            Some((count, task))
        }
    } else {
        Some((AUTOCHAT_DEFAULT_AGENTS, rest))
    }
}

fn resolve_autochat_model(
    cli_model: Option<&str>,
    env_model: Option<&str>,
    config_model: Option<&str>,
    easy_go_requested: bool,
) -> String {
    if let Some(model) = cli_model.filter(|value| !value.trim().is_empty()) {
        return model.to_string();
    }
    if easy_go_requested {
        return GO_DEFAULT_MODEL.to_string();
    }
    if let Some(model) = env_model.filter(|value| !value.trim().is_empty()) {
        return model.to_string();
    }
    if let Some(model) = config_model.filter(|value| !value.trim().is_empty()) {
        return model.to_string();
    }
    "zai/glm-5".to_string()
}

fn build_relay_profiles(count: usize) -> Vec<RelayProfile> {
    let mut profiles = Vec::with_capacity(count);
    for idx in 0..count {
        let name = format!("auto-agent-{}", idx + 1);

        let instructions = format!(
            "You are @{name}.\n\
             Role policy: self-organize from task context and current handoff instead of assuming a fixed persona.\n\
             Mission: advance the relay with concrete, high-signal next actions and clear ownership boundaries.\n\n\
             This is a protocol-first relay conversation. Treat the incoming handoff as authoritative context.\n\
             Keep your response concise, concrete, and useful for the next specialist.\n\
             Include one clear recommendation for what the next agent should do.\n\
             If the task scope is too large, explicitly call out missing specialties and handoff boundaries.",
        );
        let capabilities = vec![
            "generalist".to_string(),
            "self-organizing".to_string(),
            "relay".to_string(),
            "context-handoff".to_string(),
            "autochat".to_string(),
        ];

        profiles.push(RelayProfile {
            name,
            instructions,
            capabilities,
        });
    }
    profiles
}

fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            output.push(ch);
        } else {
            return value.to_string();
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}

fn normalize_for_convergence(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len().min(512));
    let mut last_was_space = false;

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
            last_was_space = false;
        } else if ch.is_whitespace() && !last_was_space {
            normalized.push(' ');
            last_was_space = true;
        }

        if normalized.len() >= 280 {
            break;
        }
    }

    normalized.trim().to_string()
}

async fn run_protocol_first_relay(
    agent_count: usize,
    task: &str,
    model_ref: &str,
    okr_id: Option<Uuid>,
    okr_run_id: Option<Uuid>,
) -> Result<AutochatCliResult> {
    let bus = AgentBus::new().into_arc();

    // Auto-start S3 sink if MinIO is configured
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    let relay = ProtocolRelayRuntime::new(bus.clone());

    let registry = crate::provider::ProviderRegistry::from_vault()
        .await
        .ok()
        .map(std::sync::Arc::new);

    let mut planner_used = false;
    let profiles = if let Some(registry) = &registry {
        if let Some(planned) =
            plan_relay_profiles_with_registry(task, model_ref, agent_count, registry).await
        {
            planner_used = true;
            planned
        } else {
            build_relay_profiles(agent_count)
        }
    } else {
        build_relay_profiles(agent_count)
    };

    let relay_profiles: Vec<RelayAgentProfile> = profiles
        .iter()
        .map(|profile| RelayAgentProfile {
            name: profile.name.clone(),
            capabilities: profile.capabilities.clone(),
        })
        .collect();

    let mut ordered_agents: Vec<String> = profiles
        .iter()
        .map(|profile| profile.name.clone())
        .collect();
    let mut sessions: HashMap<String, Session> = HashMap::new();

    for profile in &profiles {
        let mut session = Session::new().await?;
        session.metadata.model = Some(model_ref.to_string());
        session.agent = profile.name.clone();
        session.bus = Some(bus.clone());
        session.add_message(Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: profile.instructions.clone(),
            }],
        });
        sessions.insert(profile.name.clone(), session);
    }

    if ordered_agents.len() < 2 {
        anyhow::bail!("Autochat needs at least 2 agents to relay.");
    }

    relay.register_agents(&relay_profiles);

    // Load KR targets if OKR is associated
    let kr_targets: std::collections::HashMap<String, f64> =
        if let (Some(okr_id_val), Some(_run_id)) = (okr_id, okr_run_id) {
            if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                if let Ok(Some(okr)) = repo.get_okr(okr_id_val).await {
                    okr.key_results
                        .iter()
                        .map(|kr| (kr.id.to_string(), kr.target_value))
                        .collect()
                } else {
                    std::collections::HashMap::new()
                }
            } else {
                std::collections::HashMap::new()
            }
        } else {
            std::collections::HashMap::new()
        };

    let mut kr_progress: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

    let mut baton = format!(
        "Task:\n{task}\n\nStart by proposing an execution strategy and one immediate next step."
    );
    let mut previous_normalized: Option<String> = None;
    let mut convergence_hits = 0usize;
    let mut turns = 0usize;
    let mut dynamic_spawn_count = 0usize;
    let mut status = "max_rounds_reached".to_string();
    let mut failure_note: Option<String> = None;

    'relay_loop: for round in 1..=AUTOCHAT_MAX_ROUNDS {
        let mut idx = 0usize;
        while idx < ordered_agents.len() {
            let to = ordered_agents[idx].clone();
            let from = if idx == 0 {
                if round == 1 {
                    "user".to_string()
                } else {
                    ordered_agents[ordered_agents.len() - 1].clone()
                }
            } else {
                ordered_agents[idx - 1].clone()
            };

            turns += 1;
            relay.send_handoff(&from, &to, &baton);

            let Some(mut session) = sessions.remove(&to) else {
                status = "agent_error".to_string();
                failure_note = Some(format!("Relay agent @{to} session was unavailable."));
                break 'relay_loop;
            };

            let output = match session.prompt(&baton).await {
                Ok(response) => response.text,
                Err(err) => {
                    status = "agent_error".to_string();
                    failure_note = Some(format!("Relay agent @{to} failed: {err}"));
                    sessions.insert(to, session);
                    break 'relay_loop;
                }
            };

            sessions.insert(to.clone(), session);

            let normalized = normalize_for_convergence(&output);
            if previous_normalized.as_deref() == Some(normalized.as_str()) {
                convergence_hits += 1;
            } else {
                convergence_hits = 0;
            }
            previous_normalized = Some(normalized);

            baton = format!(
                "Relay task:\n{task}\n\nIncoming handoff from @{to}:\n{}\n\nContinue the work from this handoff. Keep your response focused and provide one concrete next-step instruction for the next agent.",
                truncate_with_ellipsis(&output, 3_500)
            );

            // Update KR progress after each turn
            if !kr_targets.is_empty() {
                let max_turns = ordered_agents.len() * AUTOCHAT_MAX_ROUNDS;
                let progress_ratio = (turns as f64 / max_turns as f64).min(1.0);

                for (kr_id, target) in &kr_targets {
                    let current = progress_ratio * target;
                    let existing = kr_progress.get(kr_id).copied().unwrap_or(0.0);
                    if current > existing {
                        kr_progress.insert(kr_id.clone(), current);
                    }
                }

                // Persist mid-run (best-effort)
                if let Some(run_id) = okr_run_id {
                    if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                        if let Ok(Some(mut run)) = repo.get_run(run_id).await {
                            if run.is_resumable() {
                                run.iterations = turns as u32;
                                for (kr_id, value) in &kr_progress {
                                    run.update_kr_progress(kr_id, *value);
                                }
                                run.status = crate::okr::OkrRunStatus::Running;
                                let _ = repo.update_run(run).await;
                            }
                        }
                    }
                }
            }

            let can_attempt_spawn = dynamic_spawn_count < AUTOCHAT_MAX_DYNAMIC_SPAWNS
                && ordered_agents.len() < AUTOCHAT_MAX_AGENTS
                && output.len() >= AUTOCHAT_SPAWN_CHECK_MIN_CHARS;

            if can_attempt_spawn
                && let Some(registry) = &registry
                && let Some((profile, reason)) = decide_dynamic_spawn_with_registry(
                    task,
                    model_ref,
                    &output,
                    round,
                    &ordered_agents,
                    registry,
                )
                .await
            {
                match Session::new().await {
                    Ok(mut spawned_session) => {
                        spawned_session.metadata.model = Some(model_ref.to_string());
                        spawned_session.agent = profile.name.clone();
                        spawned_session.bus = Some(bus.clone());
                        spawned_session.add_message(Message {
                            role: Role::System,
                            content: vec![ContentPart::Text {
                                text: profile.instructions.clone(),
                            }],
                        });

                        relay.register_agents(&[RelayAgentProfile {
                            name: profile.name.clone(),
                            capabilities: profile.capabilities.clone(),
                        }]);

                        ordered_agents.insert(idx + 1, profile.name.clone());
                        sessions.insert(profile.name.clone(), spawned_session);
                        dynamic_spawn_count += 1;

                        tracing::info!(
                            agent = %profile.name,
                            reason = %reason,
                            "Dynamic relay spawn accepted"
                        );
                    }
                    Err(err) => {
                        tracing::warn!(
                            agent = %profile.name,
                            error = %err,
                            "Dynamic relay spawn requested but failed"
                        );
                    }
                }
            }

            if convergence_hits >= 2 {
                status = "converged".to_string();
                break 'relay_loop;
            }

            idx += 1;
        }
    }

    relay.shutdown_agents(&ordered_agents);

    // Update OKR run with final progress if associated
    if let Some(run_id) = okr_run_id {
        if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
            if let Ok(Some(mut run)) = repo.get_run(run_id).await {
                // Update KR progress from execution
                for (kr_id, value) in &kr_progress {
                    run.update_kr_progress(kr_id, *value);
                }

                // Create outcomes per KR with progress (link to actual KR IDs)
                let base_evidence = vec![
                    format!("relay:{}", relay.relay_id()),
                    format!("turns:{}", turns),
                    format!("agents:{}", ordered_agents.len()),
                    format!("status:{}", status),
                ];

                let outcome_type = if status == "converged" {
                    crate::okr::KrOutcomeType::FeatureDelivered
                } else {
                    crate::okr::KrOutcomeType::Evidence
                };

                // Create one outcome per KR, linked to the actual KR ID
                for (kr_id_str, value) in &kr_progress {
                    // Parse KR ID with guardrail to prevent NIL UUID linkage
                    if let Some(kr_uuid) =
                        parse_uuid_guarded(kr_id_str, "cli_relay_outcome_kr_link")
                    {
                        let kr_description = format!(
                            "CLI relay outcome for KR {}: {} agents, {} turns, status={}",
                            kr_id_str,
                            ordered_agents.len(),
                            turns,
                            status
                        );
                        run.outcomes.push({
                            let mut outcome = crate::okr::KrOutcome::new(kr_uuid, kr_description)
                                .with_value(*value);
                            outcome.run_id = Some(run.id);
                            outcome.outcome_type = outcome_type;
                            outcome.evidence = base_evidence.clone();
                            outcome.source = "cli relay".to_string();
                            outcome
                        });
                    }
                }

                // Mark complete or update status based on execution result
                if status == "converged" {
                    run.complete();
                } else if status == "agent_error" {
                    run.status = crate::okr::OkrRunStatus::Failed;
                } else {
                    run.status = crate::okr::OkrRunStatus::Completed;
                }
                let _ = repo.update_run(run).await;
            }
        }
    }

    let mut summary = format!(
        "Autochat complete ({status}) — relay {} with {} agents over {} turns.\n\nFinal relay handoff:\n{}",
        relay.relay_id(),
        ordered_agents.len(),
        turns,
        truncate_with_ellipsis(&baton, 4_000)
    );
    if let Some(note) = &failure_note {
        summary.push_str(&format!("\n\nFailure detail: {note}"));
    }
    if planner_used {
        summary.push_str("\n\nTeam planning: model-organized profiles.");
    } else {
        summary.push_str("\n\nTeam planning: fallback self-organizing profiles.");
    }
    if dynamic_spawn_count > 0 {
        summary.push_str(&format!("\nDynamic relay spawns: {dynamic_spawn_count}"));
    }

    Ok(AutochatCliResult {
        status,
        relay_id: relay.relay_id().to_string(),
        model: model_ref.to_string(),
        agent_count: ordered_agents.len(),
        turns,
        agents: ordered_agents,
        final_handoff: baton,
        summary,
        failure: failure_note,
    })
}

#[cfg(test)]
mod tests {
    use super::PlannedRelayProfile;
    use super::{
        AUTOCHAT_QUICK_DEMO_TASK, PlannedRelayResponse, build_runtime_profile_from_plan,
        command_with_optional_args, extract_json_payload, is_easy_go_command,
        normalize_cli_go_command, parse_autochat_args, resolve_autochat_model,
    };

    #[test]
    fn normalize_go_maps_to_autochat_with_count_and_task() {
        assert_eq!(
            normalize_cli_go_command("/go 4 build protocol relay"),
            "/autochat 4 build protocol relay"
        );
    }

    #[test]
    fn normalize_go_count_only_uses_demo_task() {
        assert_eq!(
            normalize_cli_go_command("/go 4"),
            format!("/autochat 4 {AUTOCHAT_QUICK_DEMO_TASK}")
        );
    }

    #[test]
    fn parse_autochat_args_supports_default_count() {
        assert_eq!(
            parse_autochat_args("build a relay").expect("valid args"),
            (3, "build a relay")
        );
    }

    #[test]
    fn parse_autochat_args_supports_explicit_count() {
        assert_eq!(
            parse_autochat_args("4 build a relay").expect("valid args"),
            (4, "build a relay")
        );
    }

    #[test]
    fn command_with_optional_args_avoids_prefix_collision() {
        assert_eq!(command_with_optional_args("/autochatty", "/autochat"), None);
    }

    #[test]
    fn easy_go_detection_handles_aliases() {
        assert!(is_easy_go_command("/go 4 task"));
        assert!(is_easy_go_command("/team 4 task"));
        assert!(!is_easy_go_command("/autochat 4 task"));
    }

    #[test]
    fn easy_go_defaults_to_minimax_when_model_not_set() {
        assert_eq!(
            resolve_autochat_model(None, None, Some("zai/glm-5"), true),
            "minimax-credits/MiniMax-M2.5-highspeed"
        );
    }

    #[test]
    fn explicit_model_wins_over_easy_go_default() {
        assert_eq!(
            resolve_autochat_model(Some("zai/glm-5"), None, None, true),
            "zai/glm-5"
        );
    }

    #[test]
    fn extract_json_payload_parses_markdown_wrapped_json() {
        let wrapped = "Here is the plan:\n```json\n{\"profiles\":[{\"name\":\"auto-db\",\"specialty\":\"database\",\"mission\":\"Own schema and queries\",\"capabilities\":[\"sql\",\"indexing\"]}]}\n```";
        let parsed: PlannedRelayResponse =
            extract_json_payload(wrapped).expect("should parse wrapped JSON");
        assert_eq!(parsed.profiles.len(), 1);
        assert_eq!(parsed.profiles[0].name, "auto-db");
    }

    #[test]
    fn build_runtime_profile_normalizes_and_deduplicates_name() {
        let planned = PlannedRelayProfile {
            name: "Data Specialist".to_string(),
            specialty: "data engineering".to_string(),
            mission: "Prepare datasets for downstream coding".to_string(),
            capabilities: vec!["ETL".to_string(), "sql".to_string()],
        };

        let profile =
            build_runtime_profile_from_plan(planned, &["auto-data-specialist".to_string()])
                .expect("profile should be built");

        assert_eq!(profile.name, "auto-data-specialist-2");
        assert!(profile.capabilities.iter().any(|cap| cap == "relay"));
        assert!(profile.capabilities.iter().any(|cap| cap == "autochat"));
    }
}
