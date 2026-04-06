//! Formatting utilities, agent identity, clipboard, and view mode helpers

use super::*;

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

fn get_clipboard_image() -> Option<PendingImage> {
    use arboard::Clipboard;
    use image::{ImageBuffer, Rgba};
    use std::io::Cursor;

    let mut clipboard = Clipboard::new().ok()?;
    let img_data = clipboard.get_image().ok()?;

    // arboard gives us RGBA bytes
    let width = img_data.width;
    let height = img_data.height;
    let raw_bytes = img_data.bytes.into_owned();
    let size_bytes = raw_bytes.len();

    // Create an image buffer from the raw RGBA bytes
    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width as u32, height as u32, raw_bytes)?;

    // Encode as PNG
    let mut png_bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);
    img_buffer
        .write_to(&mut cursor, image::ImageFormat::Png)
        .ok()?;

    // Base64 encode
    let base64_data =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_bytes);
    let data_url = format!("data:image/png;base64,{}", base64_data);

    Some(PendingImage {
        data_url,
        width,
        height,
        size_bytes,
    })
}


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
fn read_file_preview_lines(
    path: &Path,
    max_bytes: usize,
    max_lines: usize,
) -> Result<(Vec<String>, bool, bool)> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = vec![0_u8; max_bytes.saturating_add(1)];
    let bytes_read = file.read(&mut buffer)?;

    let truncated_by_bytes = bytes_read > max_bytes;
    buffer.truncate(bytes_read.min(max_bytes));

    if buffer.contains(&0) {
        return Ok((Vec::new(), truncated_by_bytes, true));
    }

    let text = String::from_utf8_lossy(&buffer).to_string();
    let mut lines: Vec<String> = text
        .lines()
        .map(|line| truncate_with_ellipsis(line, 220))
        .collect();

    if lines.is_empty() {
        lines.push("(empty file)".to_string());
    }

    let mut truncated = truncated_by_bytes;
    if lines.len() > max_lines {
        lines.truncate(max_lines);
        truncated = true;
    }

    Ok((lines, truncated, false))
}

fn display_path_for_workspace(path: &Path, workspace_dir: &Path) -> String {
    path.strip_prefix(workspace_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

/// Build a text snippet that can be inserted into the composer to share a file with the model.
/// Returns (snippet, truncated, binary).
fn build_file_share_snippet(
    path: &Path,
    display_path: &str,
    max_bytes: usize,
) -> Result<(String, bool, bool)> {
    let bytes = std::fs::read(path)?;

    if bytes.contains(&0) {
        let snippet = format!(
            "Shared file: {display_path}\n[binary file, size: {}]",
            format_bytes(bytes.len() as u64)
        );
        return Ok((snippet, false, true));
    }

    let mut text = String::from_utf8_lossy(&bytes).to_string();
    let mut truncated = false;
    if text.len() > max_bytes {
        let mut end = max_bytes;
        while end > 0 && !text.is_char_boundary(end) {
            end -= 1;
        }
        text.truncate(end);
        truncated = true;
    }

    let language = path
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty())
        .unwrap_or("text");

    let mut snippet = format!("Shared file: {display_path}\n~~~{language}\n{text}\n~~~");
    if truncated {
        snippet.push_str(&format!(
            "\n[truncated to first {} bytes; original size: {}]",
            max_bytes,
            format_bytes(bytes.len() as u64)
        ));
    }

    Ok((snippet, truncated, false))
}

fn message_clipboard_text(message: &ChatMessage) -> String {
    let mut prefix = String::new();
    if let Some(agent) = &message.agent_name {
        prefix = format!("@{agent}\n");
    }

    match &message.message_type {
        MessageType::Text(text) => format!("{prefix}{text}"),
        MessageType::Thinking(text) => format!("{prefix}{text}"),
        MessageType::Image { url, .. } => format!("{prefix}{url}"),
        MessageType::File { path, .. } => format!("{prefix}{path}"),
        MessageType::ToolCall {
            name,
            arguments_preview,
            ..
        } => format!("{prefix}Tool call: {name}\n{arguments_preview}"),
        MessageType::ToolResult {
            name,
            output_preview,
            ..
        } => format!("{prefix}Tool result: {name}\n{output_preview}"),
    }
}

fn copy_text_to_clipboard_best_effort(text: &str) -> Result<&'static str, String> {
    if text.trim().is_empty() {
        return Err("empty text".to_string());
    }

    // 1) Try system clipboard first (works locally when a clipboard provider is available)
    match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text.to_string())) {
        Ok(()) => return Ok("system clipboard"),
        Err(e) => {
            tracing::debug!(error = %e, "System clipboard unavailable; falling back to OSC52");
        }
    }

    // 2) Fallback: OSC52 (works in many terminals, including remote SSH sessions)
    osc52_copy(text).map_err(|e| format!("osc52 copy failed: {e}"))?;
    Ok("OSC52")
}

fn osc52_copy(text: &str) -> std::io::Result<()> {

fn extract_semantic_handoff_from_rlm(answer: &str) -> String {
    match FinalPayload::parse(answer) {
        FinalPayload::Semantic(payload) => payload.answer,
        _ => answer.trim().to_string(),
    }
}

/// Ralph worker for TUI `/go` approval flow.
///
/// Loads a provider, generates a PRD, runs the Ralph loop, and reports
/// progress back to the TUI via the `AutochatUiEvent` channel.

const ALL_VIEW_MODES: [ViewMode; 9] = [
    ViewMode::Chat,
    ViewMode::Swarm,
    ViewMode::Ralph,
    ViewMode::BusLog,
    ViewMode::Protocol,
    ViewMode::SessionPicker,
    ViewMode::ModelPicker,
    ViewMode::AgentPicker,
    ViewMode::FilePicker,
];

fn view_mode_display_name(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Chat => "Chat",
        ViewMode::Swarm => "Swarm",
        ViewMode::Ralph => "Ralph",
        ViewMode::BusLog => "Bus Log",
        ViewMode::Protocol => "Protocol",
        ViewMode::SessionPicker => "Session Picker",
        ViewMode::ModelPicker => "Model Picker",
        ViewMode::AgentPicker => "Agent Picker",
        ViewMode::FilePicker => "File Picker",
    }
}

fn view_mode_shortcut_hint(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Chat => "Default (Esc from any overlay)",
        ViewMode::Swarm => "Ctrl+S / /view / F2",
        ViewMode::Ralph => "/ralph",
        ViewMode::BusLog => "Ctrl+L / F4 / /buslog",
        ViewMode::Protocol => "Ctrl+P / /protocol",
        ViewMode::SessionPicker => "/sessions",
        ViewMode::ModelPicker => "Ctrl+M / /model",
        ViewMode::AgentPicker => "Ctrl+A / /agent",
        ViewMode::FilePicker => "Ctrl+O / /file",
    }
}

fn view_mode_help_rows() -> Vec<String> {
    ALL_VIEW_MODES
        .into_iter()
        .map(|mode| {
            format!(
                "{:<14} {}",
                view_mode_display_name(mode),
                view_mode_shortcut_hint(mode)
            )
        })
        .collect()
}

fn view_mode_compact_summary() -> String {
    ALL_VIEW_MODES
        .into_iter()
        .map(view_mode_display_name)
        .collect::<Vec<_>>()
        .join(" | ")
}

fn render_help_overlay_if_needed(f: &mut Frame, app: &App, theme: &Theme) {
    if !app.show_help {
        return;
    }

    let area = centered_rect(60, 60, f.area());
    f.render_widget(Clear, area);

    let token_display = TokenDisplay::new();
    let token_info = token_display.create_detailed_display();

    // Model / provider info
    let model_section: Vec<String> = if let Some(ref active) = app.active_model {
        let (provider, model) = crate::provider::parse_model_string(active);
        let provider_label = provider.unwrap_or("auto");
        vec![
            "".to_string(),
            "  ACTIVE MODEL".to_string(),
            "  ==============".to_string(),
            format!("  Provider:  {}", provider_label),
            format!("  Model:     {}", model),
            format!("  Agent:     {}", app.current_agent),
        ]
    } else {
        vec![
            "".to_string(),
            "  ACTIVE MODEL".to_string(),
            "  ==============".to_string(),
            format!("  Provider:  auto"),
            format!("  Model:     (default)"),
            format!("  Agent:     {}", app.current_agent),
        ]
    };

    let mut help_text: Vec<String> = vec![
        "".to_string(),
        "  KEYBOARD SHORTCUTS".to_string(),
        "  ==================".to_string(),
        "".to_string(),
        "  Enter        Send message".to_string(),
        "  Tab          Switch between build/plan agents".to_string(),
        "  Ctrl+A       Open spawned-agent picker".to_string(),
        "  Ctrl+M       Open model picker".to_string(),
        "  Ctrl+O       Open file picker (attach file to composer)".to_string(),
        "  Ctrl+L       Protocol bus log".to_string(),
        "  F4           Protocol bus log".to_string(),
        "  Ctrl+P       Protocol registry".to_string(),
        "  Ctrl+S       Toggle swarm view".to_string(),
        "  Ctrl+B       Toggle webview layout".to_string(),
        "  Ctrl+V       Paste image from clipboard".to_string(),
        "  Ctrl+Y       Copy latest assistant reply".to_string(),
        "  F3           Toggle inspector pane".to_string(),
        "  Ctrl+C       Quit".to_string(),
        "  ?            Toggle this help".to_string(),
        "".to_string(),
        "  SLASH COMMANDS (auto-complete hints shown while typing)".to_string(),
        "  OKR/PRD-GATED MODE (requires approval, tracks measurable outcomes)".to_string(),
        "  /go <task>      OKR+PRD relay: draft → approve → execute → track KR progress"
            .to_string(),
        "".to_string(),
        "  RELAY MODE".to_string(),
        "  /autochat [count] [--no-prd] <task>  PRD-gated by default; use --no-prd for tactical"
            .to_string(),
        "  /autochat-local [count] [--no-prd] <task>  Local relay; PRD-gated unless --no-prd"
            .to_string(),
        "  /local [model]  Switch active model to local CUDA (example: /local qwen3.5-9b)"
            .to_string(),
        "".to_string(),
        "  EASY MODE".to_string(),
        "  /add <name>     Create a helper teammate".to_string(),
        "  /talk <name> <message>  Message teammate".to_string(),
        "  /list           List teammates".to_string(),
        "  /remove <name>  Remove teammate".to_string(),
        "  /home           Return to main chat".to_string(),
        "  /help           Open this help".to_string(),
        "".to_string(),
        "  ADVANCED MODE".to_string(),
        "  /spawn <name> <instructions>  Create a named sub-agent".to_string(),
        "  /agents        List spawned sub-agents".to_string(),
        "  /kill <name>   Remove a spawned sub-agent".to_string(),
        "  /agent <name>  Focus chat on a spawned sub-agent".to_string(),
        "  /agent <name> <message>  Send one message to a spawned sub-agent".to_string(),
        "  /agent            Open spawned-agent picker".to_string(),
        "  /agent main|off  Exit focused sub-agent chat".to_string(),
        "  /swarm <task>   Run task in parallel swarm mode".to_string(),
        "  /ralph [path]   Start Ralph PRD loop (default: prd.json)".to_string(),
        "  /undo           Undo last message and response".to_string(),
        "  /sessions       Open session picker (filter, delete, load, n/p paginate)".to_string(),
        "  /resume         Resume interrupted relay or most recent session".to_string(),
        "  /resume <id>    Resume specific session by ID".to_string(),
        "  /new            Start a fresh session".to_string(),
        "  /model          Open model picker (or /model <name>)".to_string(),
        "  /file           Open file picker (or /file <path>)".to_string(),
        "  /view           Toggle swarm view".to_string(),
        "  /buslog         Show protocol bus log".to_string(),
        "  /protocol       Show protocol registry and AgentCards".to_string(),
        "  /webview        Web dashboard layout".to_string(),
        "  /classic        Single-pane layout".to_string(),
        "  /inspector      Toggle inspector pane".to_string(),
        "  /refresh        Refresh workspace and sessions".to_string(),
        "  /archive        Show persistent chat archive path".to_string(),
        "".to_string(),
        "  CLI AUTONOMY HELPERS (run from shell)".to_string(),
        "  codetether forage --top 5".to_string(),
        "  codetether forage --loop --interval-secs 120 --top 3".to_string(),
        "  codetether forage --loop --execute --interval-secs 120 --top 3".to_string(),
        "".to_string(),
        "  SESSION PICKER".to_string(),
        "  ↑/↓/j/k      Navigate sessions".to_string(),
        "  Enter         Load selected session".to_string(),
        "  d             Delete session (press twice to confirm)".to_string(),
        "  Type          Filter sessions by name/agent/ID".to_string(),
        "  Backspace     Clear filter character".to_string(),
        "  Esc           Close picker".to_string(),
        "".to_string(),
        "  FILE PICKER".to_string(),
        "  ↑/↓/j/k      Navigate files".to_string(),
        "  PgUp/PgDn    Jump by page".to_string(),
        "  Home/End     Jump to top/bottom".to_string(),
        "  Enter/l       Open folder or attach selected file to composer".to_string(),
        "  h/Left        Parent directory".to_string(),
        "  Type          Filter file list".to_string(),
        "  Ctrl+U        Clear filter".to_string(),
        "  Backspace     Clear filter / go parent".to_string(),
        "  F5            Refresh listing".to_string(),
        "  Preview pane  Shows selected file/folder details".to_string(),
        "  Esc           Close picker".to_string(),
        "".to_string(),
        "  VIM-STYLE NAVIGATION".to_string(),
        "  Alt+j        Scroll down".to_string(),
        "  Alt+k        Scroll up".to_string(),
        "  Ctrl+g       Go to top".to_string(),
        "  Ctrl+G       Follow latest".to_string(),
        "".to_string(),
        "  SCROLLING".to_string(),
        "  Up/Down      Scroll messages".to_string(),
        "  PageUp/Dn    Scroll one page".to_string(),
        "  Alt+u/d      Scroll half page".to_string(),
        "".to_string(),
        "  COMMAND HISTORY".to_string(),
        "  Ctrl+R       Search history".to_string(),
        "  Ctrl+Up/Dn   Navigate history".to_string(),
        "".to_string(),
        "  Press ? or Esc to close".to_string(),
        "".to_string(),
    ];

    help_text.push("  VIEW MODES".to_string());
    help_text.extend(
        view_mode_help_rows()
            .into_iter()
            .map(|line| format!("  {line}")),
    );
    help_text.push("".to_string());

    let mut combined_text = token_info;
    combined_text.extend(model_section);
    combined_text.extend(help_text);

    let help = Paragraph::new(combined_text.join("\n"))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .border_style(Style::default().fg(theme.help_border_color.to_color())),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(help, area);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

