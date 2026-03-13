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
    match input.trim() {
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
    }
}
