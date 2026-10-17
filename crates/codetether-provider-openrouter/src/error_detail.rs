//! Extraction of the upstream error hidden in OpenRouter metadata.

use serde_json::Value;

/// Returns a concise provider-specific diagnostic from `error.metadata`.
pub(super) fn extract(metadata: &Value) -> Option<String> {
    let raw = metadata.get("raw")?.as_str()?;
    let decoded = serde_json::from_str::<Value>(raw).ok()?;
    let upstream = decoded
        .pointer("/error/message")
        .or_else(|| decoded.get("message"))?
        .as_str()?;
    let provider = metadata
        .get("provider_name")
        .and_then(Value::as_str)
        .unwrap_or("upstream provider");
    Some(format!("{provider}: {}", compact(upstream)))
}

fn compact(message: &str) -> String {
    const LIMIT: usize = 500;
    let one_line = message.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.len() <= LIMIT {
        return one_line;
    }
    let end = one_line
        .char_indices()
        .map(|(index, _)| index)
        .take_while(|index| *index <= LIMIT)
        .last()
        .unwrap_or(0);
    format!("{}…", &one_line[..end])
}
