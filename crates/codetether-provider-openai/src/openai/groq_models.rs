//! Keep active Groq text-generation models, excluding speech and guard models.
use serde_json::Value;

pub(super) fn is_chat(entry: &Value) -> bool {
    if entry.get("active").and_then(Value::as_bool) == Some(false) {
        return false;
    }
    let id = entry
        .get("id")
        .and_then(Value::as_str)
        .or_else(|| entry.as_str())
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let leaf = id.rsplit('/').next().unwrap_or("");
    !leaf.is_empty()
        && ![
            "whisper-",
            "distil-whisper-",
            "playai-",
            "orpheus-",
            "llama-guard-",
            "llama-prompt-guard-",
            "gpt-oss-safeguard-",
        ]
        .iter()
        .any(|prefix| leaf.starts_with(prefix))
}
