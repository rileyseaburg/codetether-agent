//! Extended slash command parsing, hints, and easy-mode aliases


fn match_slash_command_hint(input: &str) -> String {
    let commands = [
        (
            "/go ",
            "OKR-gated relay (requires approval, tracks outcomes)",
        ),
        ("/add ", "Easy mode: create a teammate"),
        ("/talk ", "Easy mode: message or focus a teammate"),
        ("/list", "Easy mode: list teammates"),
        ("/remove ", "Easy mode: remove a teammate"),
        ("/home", "Easy mode: return to main chat"),
        ("/help", "Open help"),
        ("/spawn ", "Create a named sub-agent"),
        ("/autochat ", "Tactical relay (fast path, no OKR tracking)"),
        (
            "/autochat-local ",
            "Tactical relay forced to local CUDA model",
        ),
        ("/local", "Switch active model to local CUDA"),
        ("/agents", "List spawned sub-agents"),
        ("/kill ", "Remove a spawned sub-agent"),
        ("/agent ", "Focus or message a spawned sub-agent"),
        ("/swarm ", "Run task in parallel swarm mode"),
        ("/ralph", "Start autonomous PRD loop"),
        ("/undo", "Undo last message and response"),
        ("/sessions", "Open session picker"),
        ("/resume", "Resume session or interrupted relay"),
        ("/new", "Start a fresh session"),
        ("/model", "Select or set model"),
        ("/file", "Open file picker or attach /file <path>"),
        ("/webview", "Switch to webview layout"),
        ("/classic", "Switch to classic layout"),
        ("/inspector", "Toggle inspector pane"),
        ("/refresh", "Refresh workspace"),
        ("/archive", "Show persistent chat archive path"),
        (
            "/view",
            "Toggle Chat/Swarm view (see /help for all view modes)",
        ),
        ("/buslog", "Show protocol bus log"),
        ("/protocol", "Show protocol registry"),
    ];

    let trimmed = input.trim_start();
    let input_lower = trimmed.to_lowercase();

    // Exact command (or command + args) should always resolve to one hint.
    if let Some((cmd, desc)) = commands.iter().find(|(cmd, _)| {
        let key = cmd.trim_end().to_ascii_lowercase();
        input_lower == key || input_lower.starts_with(&(key + " "))
    }) {
        return format!("{} — {}", cmd.trim(), desc);
    }

    // Fallback to prefix matching while the user is still typing.
    let matches: Vec<_> = commands
        .iter()
        .filter(|(cmd, _)| cmd.starts_with(&input_lower))
        .collect();

    if matches.len() == 1 {
        format!("{} — {}", matches[0].0.trim(), matches[0].1)
    } else if matches.is_empty() {
        "Unknown command".to_string()
    } else {
        let cmds: Vec<_> = matches.iter().map(|(cmd, _)| cmd.trim()).collect();
        cmds.join(" | ")
    }
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

fn normalize_easy_command(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if !trimmed.starts_with('/') {
        return input.to_string();
    }

    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("").trim();

    match command.to_ascii_lowercase().as_str() {
        "/go" | "/team" => {
            if args.is_empty() {
                "/autochat".to_string()
            } else {
                let mut parts = args.splitn(2, char::is_whitespace);
                let first = parts.next().unwrap_or("").trim();
                if let Ok(count) = first.parse::<usize>() {
                    let rest = parts.next().unwrap_or("").trim();
                    if rest.is_empty() {
                        format!("/autochat {count} {AUTOCHAT_QUICK_DEMO_TASK}")
                    } else {
                        format!("/autochat {count} {rest}")
                    }
                } else {
                    format!("/autochat {AUTOCHAT_DEFAULT_AGENTS} {args}")
                }
            }
        }
        "/add" => {
            if args.is_empty() {
                "/spawn".to_string()
            } else {
                format!("/spawn {args}")
            }
        }
        "/list" | "/ls" => "/agents".to_string(),
        "/remove" | "/rm" => {
            if args.is_empty() {
                "/kill".to_string()
            } else {
                format!("/kill {args}")
            }
        }
        "/talk" | "/say" => {
            if args.is_empty() {
                "/agent".to_string()
            } else {
                format!("/agent {args}")
            }
        }
        "/focus" => {
            if args.is_empty() {
                "/agent".to_string()
            } else {
                format!("/agent {}", args.trim_start_matches('@'))
            }
        }
        "/home" | "/main" => "/agent main".to_string(),
        "/h" | "/?" => "/help".to_string(),
        _ => trimmed.to_string(),
    }
}

fn is_easy_go_command(input: &str) -> bool {
    let command = input
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(command.as_str(), "/go" | "/team")
}

