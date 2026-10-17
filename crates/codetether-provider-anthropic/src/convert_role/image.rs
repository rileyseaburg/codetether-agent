//! Native Anthropic images; unsupported references become visible text notices.
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

/// Convert an image reference for native and Vertex Anthropic content blocks.
///
/// # Arguments
/// * url - HTTP(S) image reference or encoded image data URL.
/// * _mime_type - Retained compatibility parameter; the URL determines type.
///
/// # Returns
/// An image content block, or a text notice when the reference is unsupported.
///
/// # Examples
///
///     let block = codetether_provider_anthropic::image_block("https://example.test/image.png", None);
///     assert_eq!(block["type"], "image");
pub fn block(url: &str, _mime_type: Option<&str>) -> Value {
    if url.starts_with("https://") || url.starts_with("http://") {
        return json!({"type": "image", "source": {"type": "url", "url": url}});
    }
    // Claude rejects images over 2000 px on either edge; shrink them first.
    let fitted = codetether_image::fit::fit_data_url(url);
    let url = fitted.as_deref().unwrap_or(url);
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
