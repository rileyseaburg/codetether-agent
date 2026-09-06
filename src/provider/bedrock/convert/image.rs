//! Converse ImageBlock serialization, with notices for unsupported references.
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

pub(super) fn block(url: &str, _mime_type: Option<&str>) -> Value {
    if let Some((header, data)) = url.strip_prefix("data:").and_then(|s| s.split_once(','))
        && let Some(mime) = header.strip_suffix(";base64")
        && let Some(format) = mime.strip_prefix("image/")
        && matches!(format, "png" | "jpeg" | "gif" | "webp")
        && !data.is_empty()
        && STANDARD.decode(data).is_ok()
    {
        return json!({"image": {"format": format, "source": {"bytes": data}}});
    }
    json!({"text": "[Image unavailable: Bedrock Converse conversion requires a valid base64 PNG, JPEG, GIF, or WebP data URL; remote and file URLs are not fetched.]"})
}
