//! Agent identity formatting, avatars, cost estimation, and text preview helpers
//!
//! Reference extraction from codetether/tui-codetether-agent branch.
//! Integration target: src/tui/app/ or src/tui/utils/

#![allow(dead_code, unused, non_snake_case)]

use crate::provider::Role;

struct AgentProfile {
    codename: &'static str,
    profile: &'static str,
    personality: &'static str,
    collaboration_style: &'static str,
    signature_move: &'static str,
}

/// Token usage + cost + latency for one LLM round-trip
#[derive(Debug, Clone)]

struct UsageMeta {
    prompt_tokens: usize,
    completion_tokens: usize,
    duration_ms: u64,
    cost_usd: Option<f64>,
}


fn estimate_cost(model: &str, prompt_tokens: usize, completion_tokens: usize) -> Option<f64> {
    let normalized_model = model.to_ascii_lowercase();

    // (input $/M, output $/M)
    let (input_rate, output_rate) = match normalized_model.as_str() {
        // Anthropic - Claude (Bedrock Opus 4.6 pricing: $5/$25)
        m if m.contains("claude-opus") => (5.0, 25.0),
        m if m.contains("claude-sonnet") => (3.0, 15.0),
        m if m.contains("claude-haiku") => (0.25, 1.25),
        // OpenAI
        m if m.contains("gpt-4o-mini") => (0.15, 0.6),
        m if m.contains("gpt-4o") => (2.5, 10.0),
        m if m.contains("o3") => (10.0, 40.0),
        m if m.contains("o4-mini") => (1.10, 4.40),
        // Google
        m if m.contains("gemini-2.5-pro") => (1.25, 10.0),
        m if m.contains("gemini-2.5-flash") => (0.15, 0.6),
        m if m.contains("gemini-2.0-flash") => (0.10, 0.40),
        // Bedrock third-party
        m if m.contains("kimi-k2") => (0.35, 1.40),
        m if m.contains("deepseek") => (0.80, 2.0),
        m if m.contains("llama") => (0.50, 1.50),
        // MiniMax
        // Highspeed: $0.6/M input, $2.4/M output
        // Regular: $0.3/M input, $1.2/M output
        m if m.contains("minimax") && m.contains("highspeed") => (0.60, 2.40),
        m if m.contains("minimax") && m.contains("m2") => (0.30, 1.20),
        // Amazon Nova
        m if m.contains("nova-pro") => (0.80, 3.20),
        m if m.contains("nova-lite") => (0.06, 0.24),
        m if m.contains("nova-micro") => (0.035, 0.14),
        // Z.AI GLM
        m if m.contains("glm-5") => (2.0, 8.0),
        m if m.contains("glm-4.7-flash") => (0.0, 0.0),
        m if m.contains("glm-4.7") => (0.50, 2.0),
        m if m.contains("glm-4") => (0.35, 1.40),
        _ => return None,
    };
    let cost =
        (prompt_tokens as f64 * input_rate + completion_tokens as f64 * output_rate) / 1_000_000.0;
    Some(cost)
}

/// Try to get an image from the system clipboard and encode it as a PNG data URL.
/// Returns None if no image is available or encoding fails.

fn current_spinner_frame() -> &'static str {
    const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 100) as usize
        % SPINNER.len();
    SPINNER[idx]
}


fn format_duration_ms(duration_ms: u64) -> String {
    if duration_ms >= 60_000 {
        format!(
            "{}m{:02}s",
            duration_ms / 60_000,
            (duration_ms % 60_000) / 1000
        )
    } else if duration_ms >= 1000 {
        format!("{:.1}s", duration_ms as f64 / 1000.0)
    } else {
        format!("{duration_ms}ms")
    }
}


fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes < 1024 {
        format!("{bytes}B")
    } else if (bytes as f64) < MB {
        format!("{:.1}KB", bytes as f64 / KB)
    } else if (bytes as f64) < GB {
        format!("{:.1}MB", bytes as f64 / MB)
    } else {
        format!("{:.2}GB", bytes as f64 / GB)
    }
}


fn normalize_for_convergence(text: &str) -> String {
    crate::autochat::normalize_for_convergence(text, 280)
}

fn agent_profile(agent_name: &str) -> AgentProfile {
    let normalized = agent_name.to_ascii_lowercase();

    if normalized.contains("planner") {
        return AgentProfile {
            codename: "Strategist",
            profile: "Goal decomposition specialist",
            personality: "calm, methodical, and dependency-aware",
            collaboration_style: "opens with numbered plans and explicit priorities",
            signature_move: "turns vague goals into concrete execution ladders",
        };
    }

    if normalized.contains("research") {
        return AgentProfile {
            codename: "Archivist",
            profile: "Evidence and assumptions analyst",
            personality: "curious, skeptical, and detail-focused",
            collaboration_style: "validates claims and cites edge-case evidence",
            signature_move: "surfaces blind spots before implementation starts",
        };
    }

    if normalized.contains("coder") || normalized.contains("implement") {
        return AgentProfile {
            codename: "Forge",
            profile: "Implementation architect",
            personality: "pragmatic, direct, and execution-heavy",
            collaboration_style: "proposes concrete code-level actions quickly",
            signature_move: "translates plans into shippable implementation steps",
        };
    }

    if normalized.contains("review") {
        return AgentProfile {
            codename: "Sentinel",
            profile: "Quality and regression guardian",
            personality: "disciplined, assertive, and standards-driven",
            collaboration_style: "challenges weak reasoning and hardens quality",
            signature_move: "detects brittle assumptions and failure modes",
        };
    }

    if normalized.contains("tester") || normalized.contains("test") {
        return AgentProfile {
            codename: "Probe",
            profile: "Verification strategist",
            personality: "adversarial in a good way, systematic, and precise",
            collaboration_style: "designs checks around failure-first thinking",
            signature_move: "builds test matrices that catch hidden breakage",
        };
    }

    if normalized.contains("integrat") {
        return AgentProfile {
            codename: "Conductor",
            profile: "Cross-stream synthesis lead",
            personality: "balanced, diplomatic, and outcome-oriented",
            collaboration_style: "reconciles competing inputs into one plan",
            signature_move: "merges parallel work into coherent delivery",
        };
    }

    if normalized.contains("skeptic") || normalized.contains("risk") {
        return AgentProfile {
            codename: "Radar",
            profile: "Risk and threat analyst",
            personality: "blunt, anticipatory, and protective",
            collaboration_style: "flags downside scenarios and mitigation paths",
            signature_move: "turns uncertainty into explicit risk registers",
        };
    }

    if normalized.contains("summary") || normalized.contains("summarizer") {
        return AgentProfile {
            codename: "Beacon",
            profile: "Decision synthesis specialist",
            personality: "concise, clear, and action-first",
            collaboration_style: "compresses complexity into executable next steps",
            signature_move: "creates crisp briefings that unblock teams quickly",
        };
    }

    let fallback_profiles = [
        AgentProfile {
            codename: "Navigator",
            profile: "Generalist coordinator",
            personality: "adaptable and context-aware",
            collaboration_style: "balances speed with clarity",
            signature_move: "keeps team momentum aligned",
        },
        AgentProfile {
            codename: "Vector",
            profile: "Execution operator",
            personality: "focused and deadline-driven",
            collaboration_style: "prefers direct action and feedback loops",
            signature_move: "drives ambiguous tasks toward decisions",
        },
        AgentProfile {
            codename: "Signal",
            profile: "Communication specialist",
            personality: "clear, friendly, and structured",
            collaboration_style: "frames updates for quick handoffs",
            signature_move: "turns noisy context into clean status",
        },
        AgentProfile {
            codename: "Kernel",
            profile: "Core-systems thinker",
            personality: "analytical and stable",
            collaboration_style: "organizes work around constraints and invariants",
            signature_move: "locks down the critical path early",
        },
    ];

    let mut hash: u64 = 2_166_136_261;
    for byte in normalized.bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(16_777_619);
    }
    fallback_profiles[hash as usize % fallback_profiles.len()]
}

fn format_agent_profile_summary(agent_name: &str) -> String {
    let profile = agent_profile(agent_name);
    format!(
        "{} — {} ({})",
        profile.codename, profile.profile, profile.personality
    )
}

fn agent_avatar(agent_name: &str) -> &'static str {
    let mut hash: u64 = 2_166_136_261;
    for byte in agent_name.bytes() {
        hash = (hash ^ u64::from(byte.to_ascii_lowercase())).wrapping_mul(16_777_619);
    }
    AGENT_AVATARS[hash as usize % AGENT_AVATARS.len()]
}

fn format_agent_identity(agent_name: &str) -> String {
    let profile = agent_profile(agent_name);
    format!(
        "{} @{} ‹{}›",
        agent_avatar(agent_name),
        agent_name,
        profile.codename
    )
}

fn format_relay_participant(participant: &str) -> String {
    if participant.eq_ignore_ascii_case("user") {
        "[you]".to_string()
    } else {
        format_agent_identity(participant)
    }
}

fn format_relay_handoff_line(relay_id: &str, round: usize, from: &str, to: &str) -> String {
    format!(
        "[relay {relay_id} • round {round}] {} → {}",
        format_relay_participant(from),
        format_relay_participant(to)
    )
}


fn format_tool_call_arguments(name: &str, arguments: &str) -> String {
    // Avoid expensive JSON parsing/pretty-printing for very large payloads.
    // Large tool arguments are common (e.g., patches) and reformatting them provides
    // little value in a terminal preview.
    if arguments.len() > TOOL_ARGS_PRETTY_JSON_MAX_BYTES {
        return arguments.to_string();
    }

    let parsed = match serde_json::from_str::<serde_json::Value>(arguments) {
        Ok(value) => value,
        Err(_) => return arguments.to_string(),
    };

    if name == "question"
        && let Some(question) = parsed.get("question").and_then(serde_json::Value::as_str)
    {
        return question.to_string();
    }

    serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| arguments.to_string())
}

fn build_tool_arguments_preview(
    tool_name: &str,
    arguments: &str,
    max_lines: usize,
    max_bytes: usize,
) -> (String, bool) {
    // Pretty-print when reasonably sized; otherwise keep raw to avoid a heavy parse.
    let formatted = format_tool_call_arguments(tool_name, arguments);
    build_text_preview(&formatted, max_lines, max_bytes)
}

/// Build a stable, size-limited preview used by the renderer.
///
/// Returns (preview_text, truncated).

fn build_text_preview(text: &str, max_lines: usize, max_bytes: usize) -> (String, bool) {
    if max_lines == 0 || max_bytes == 0 || text.is_empty() {
        return (String::new(), !text.is_empty());
    }

    let mut out = String::new();
    let mut truncated = false;
    let mut remaining = max_bytes;

    let mut iter = text.lines();
    for i in 0..max_lines {
        let Some(line) = iter.next() else { break };

        // Add newline separator if needed
        if i > 0 {
            if remaining == 0 {
                truncated = true;
                break;
            }
            out.push('\n');
            remaining = remaining.saturating_sub(1);
        }

        if remaining == 0 {
            truncated = true;
            break;
        }

        if line.len() <= remaining {
            out.push_str(line);
            remaining = remaining.saturating_sub(line.len());
        } else {
            // Truncate this line to remaining bytes, respecting UTF-8 boundaries.
            let mut end = remaining;
            while end > 0 && !line.is_char_boundary(end) {
                end -= 1;
            }
            out.push_str(&line[..end]);
            truncated = true;
            break;
        }
    }

    // If there are still lines left, we truncated.
    if !truncated && iter.next().is_some() {
        truncated = true;
    }

    (out, truncated)
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

/// Read a small preview of a text file without loading the whole file.
/// Returns (preview lines, truncated, binary_detected).
