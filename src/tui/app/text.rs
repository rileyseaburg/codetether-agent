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
    use super::{command_with_optional_args, normalize_slash_command};

    #[test]
    fn normalize_slash_command_preserves_arguments() {
        assert_eq!(normalize_slash_command("/p events"), "/protocol events");
    }

    #[test]
    fn command_with_optional_args_avoids_prefix_collisions() {
        assert_eq!(command_with_optional_args("/filed", "/file"), None);
    }
}
