//! Native Anthropic images; unsupported references become visible text notices.
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

/// Convert an image reference for native and Vertex Anthropic content blocks.
pub(crate) fn block(url: &str, _mime_type: Option<&str>) -> Value {
    if url.starts_with("https://") || url.starts_with("http://") {
        return json!({"type": "image", "source": {"type": "url", "url": url}});
    }
    if let Some((header, data)) = url.strip_prefix("data:").and_then(|s| s.split_once(','))
        && let Some(mime) = header.strip_suffix(";base64")
        && matches!(
            mime,
            "image/png" | "image/jpeg" | "image/gif" | "image/webp"
        )
        && !data.is_empty()
        && STANDARD.decode(data).is_ok()
    {
        return json!({"type": "image", "source": {
            "type": "base64", "media_type": mime, "data": data
        }});
    }
    json!({"type": "text", "text": "[Image unavailable: Anthropic requires an HTTP(S) URL or a valid base64 PNG, JPEG, GIF, or WebP data URL.]"})
}
