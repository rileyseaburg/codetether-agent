//! Vertex-specific user/tool content conversion with shared Anthropic images.
//! System shape, assistant conversion, credentials, and transport stay in the provider.
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod tests;
pub(super) mod tool;
pub(super) mod user;

fn image_block(url: &str, mime: Option<&str>) -> serde_json::Value {
    // Claude vision docs: Google Cloud and Bedrock only accept base64 sources.
    if url.starts_with("https://") || url.starts_with("http://") {
        return serde_json::json!({
            "type": "text",
            "text": "[Image unavailable: Vertex Anthropic requires base64 image data; remote image URLs are not fetched.]"
        });
    }
    crate::provider::anthropic::image_block(url, mime)
}
