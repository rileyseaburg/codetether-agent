pub fn truncate_preview(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let preview: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{preview}…")
    } else {
        preview
    }
}

pub fn normalize_slash_command(input: &str) -> String {
    let trimmed = input.trim();
    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("").trim();

    let normalized = match command {
        "/h" | "/?" => "/help".to_string(),
        "/s" => "/sessions".to_string(),
        "/w" => "/swarm".to_string(),
        "/r" => "/ralph".to_string(),
        "/b" | "/buslog" | "/logs" => "/bus".to_string(),
        "/p" => "/protocol".to_string(),
        "/m" => "/model".to_string(),
        "/set" => "/settings".to_string(),
        "/sym" => "/symbols".to_string(),
        other => other.to_string(),
    };

    if args.is_empty() {
        normalized
    } else {
        format!("{normalized} {args}")
    }
}

/// Easy-mode command aliases that map user-friendly names to advanced commands.
/// Call this early in the input pipeline before slash command dispatch.
pub fn normalize_easy_command(input: &str) -> String {
    let trimmed = input.trim();
    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("").trim();

    let normalized = match command {
        "/add" => {
            if args.is_empty() {
                "/spawn".to_string()
            } else {
                format!("/spawn {args}")
            }
        }
        "/talk" | "/say" => {
            if args.is_empty() {
                "/agent".to_string()
            } else {
                format!("/agent {args}")
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
        "/focus" => {
            if args.is_empty() {
                "/agent".to_string()
            } else {
                format!("/agent {}", args.trim_start_matches('@'))
            }
        }
        "/home" | "/main" => "/chat".to_string(),
        _ => return input.to_string(),
    };

    normalized
}

pub fn command_with_optional_args<'a>(input: &'a str, command: &str) -> Option<&'a str> {
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

#[cfg(test)]
mod tests {
    use super::{command_with_optional_args, normalize_easy_command, normalize_slash_command};

    #[test]
    fn normalize_slash_command_preserves_arguments() {
        assert_eq!(normalize_slash_command("/p events"), "/protocol events");
    }

    #[test]
    fn command_with_optional_args_avoids_prefix_collisions() {
        assert_eq!(command_with_optional_args("/filed", "/file"), None);
    }

    #[test]
    fn normalize_easy_command_add_alias() {
        assert_eq!(normalize_easy_command("/add coder"), "/spawn coder");
        assert_eq!(normalize_easy_command("/add"), "/spawn");
    }

    #[test]
    fn normalize_easy_command_talk_alias() {
        assert_eq!(
            normalize_easy_command("/talk coder hello"),
            "/agent coder hello"
        );
        assert_eq!(
            normalize_easy_command("/say coder hello"),
            "/agent coder hello"
        );
        assert_eq!(normalize_easy_command("/talk"), "/agent");
    }

    #[test]
    fn normalize_easy_command_list_alias() {
        assert_eq!(normalize_easy_command("/list"), "/agents");
        assert_eq!(normalize_easy_command("/ls"), "/agents");
    }

    #[test]
    fn normalize_easy_command_remove_alias() {
        assert_eq!(normalize_easy_command("/remove coder"), "/kill coder");
        assert_eq!(normalize_easy_command("/rm coder"), "/kill coder");
        assert_eq!(normalize_easy_command("/remove"), "/kill");
    }

    #[test]
    fn normalize_easy_command_focus_alias() {
        assert_eq!(normalize_easy_command("/focus coder"), "/agent coder");
        assert_eq!(normalize_easy_command("/focus @coder"), "/agent coder");
        assert_eq!(normalize_easy_command("/focus"), "/agent");
    }

    #[test]
    fn normalize_easy_command_home_alias() {
        assert_eq!(normalize_easy_command("/home"), "/chat");
        assert_eq!(normalize_easy_command("/main"), "/chat");
    }

    #[test]
    fn normalize_easy_command_passthrough() {
        assert_eq!(normalize_easy_command("/help"), "/help");
        assert_eq!(normalize_easy_command("/spawn coder"), "/spawn coder");
        assert_eq!(normalize_easy_command("hello world"), "hello world");
    }
}
