//! Non-interactive run command

use super::RunArgs;
use crate::bus::{AgentBus, relay::ProtocolRelayRuntime, relay::RelayAgentProfile};
use crate::config::Config;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;

const AUTOCHAT_MAX_AGENTS: usize = 8;
const AUTOCHAT_DEFAULT_AGENTS: usize = 3;
const AUTOCHAT_MAX_ROUNDS: usize = 3;
const AUTOCHAT_QUICK_DEMO_TASK: &str = "Introduce yourselves with your role/personality, then relay one concrete implementation plan with clear next handoffs.";
const GO_DEFAULT_MODEL: &str = "minimax/MiniMax-M2.5";

#[derive(Debug, Clone)]
struct RelayProfile {
    name: String,
    instructions: String,
    capabilities: Vec<String>,
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

        let relay_result = run_protocol_first_relay(agent_count, task, &model).await?;
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
    let templates = [
        (
            "planner",
            "Decompose objectives into precise, sequenced steps.",
            "planning",
        ),
        (
            "researcher",
            "Validate assumptions, surface edge cases, and gather critical evidence.",
            "research",
        ),
        (
            "coder",
            "Propose concrete implementation details and practical code-level direction.",
            "implementation",
        ),
        (
            "reviewer",
            "Challenge weak spots, enforce quality, and reduce regressions.",
            "review",
        ),
        (
            "tester",
            "Design verification strategy, tests, and failure-oriented checks.",
            "testing",
        ),
        (
            "integrator",
            "Synthesize contributions into a coherent delivery plan.",
            "integration",
        ),
        (
            "skeptic",
            "Stress-test confidence and call out hidden risks early.",
            "risk-analysis",
        ),
        (
            "summarizer",
            "Produce concise, actionable final guidance.",
            "summarization",
        ),
    ];

    let mut profiles = Vec::with_capacity(count);
    for idx in 0..count {
        let (slug, mission, specialty) = templates[idx % templates.len()];
        let name = if idx < templates.len() {
            format!("auto-{slug}")
        } else {
            format!("auto-{slug}-{}", idx + 1)
        };

        let instructions = format!(
            "You are @{name}.\n\
             Specialty: {specialty}. {mission}\n\n\
             This is a protocol-first relay conversation. Treat the incoming handoff as authoritative context.\n\
             Keep your response concise, concrete, and useful for the next specialist.\n\
             Include one clear recommendation for what the next agent should do.",
        );
        let capabilities = vec![
            specialty.to_string(),
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
) -> Result<AutochatCliResult> {
    let bus = AgentBus::new().into_arc();
    let relay = ProtocolRelayRuntime::new(bus);

    let profiles = build_relay_profiles(agent_count);
    let relay_profiles: Vec<RelayAgentProfile> = profiles
        .iter()
        .map(|profile| RelayAgentProfile {
            name: profile.name.clone(),
            capabilities: profile.capabilities.clone(),
        })
        .collect();

    let ordered_agents: Vec<String> = profiles
        .iter()
        .map(|profile| profile.name.clone())
        .collect();
    let mut sessions: HashMap<String, Session> = HashMap::new();

    for profile in &profiles {
        let mut session = Session::new().await?;
        session.metadata.model = Some(model_ref.to_string());
        session.agent = profile.name.clone();
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

    let mut baton = format!(
        "Task:\n{task}\n\nStart by proposing an execution strategy and one immediate next step."
    );
    let mut previous_normalized: Option<String> = None;
    let mut convergence_hits = 0usize;
    let mut turns = 0usize;
    let mut status = "max_rounds_reached".to_string();
    let mut failure_note: Option<String> = None;

    'relay_loop: for round in 1..=AUTOCHAT_MAX_ROUNDS {
        for idx in 0..ordered_agents.len() {
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

            if convergence_hits >= 2 {
                status = "converged".to_string();
                break 'relay_loop;
            }
        }
    }

    relay.shutdown_agents(&ordered_agents);

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
    use super::{
        AUTOCHAT_QUICK_DEMO_TASK, command_with_optional_args, is_easy_go_command,
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
            "minimax/MiniMax-M2.5"
        );
    }

    #[test]
    fn explicit_model_wins_over_easy_go_default() {
        assert_eq!(
            resolve_autochat_model(Some("zai/glm-5"), None, None, true),
            "zai/glm-5"
        );
    }
}
