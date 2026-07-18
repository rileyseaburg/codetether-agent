//! Convert data URLs and local image files into durable attachments.

use crate::tool::agent::collaboration_runtime::message_input::MessageImage;
use anyhow::{Context, Result, bail};
use base64::Engine;
use std::path::Path;

pub(super) fn data_url(value: &str) -> Result<MessageImage> {
    let image = crate::image_clipboard::attachment_from_data_url(value)
        .context("image_url must be a base64 image data URL")?;
    Ok(MessageImage {
        data_url: image.data_url,
        mime_type: image.mime_type,
    })
}

pub(super) async fn local(path: &Path) -> Result<MessageImage> {
    let mime = mime(path).context("unsupported local image format")?;
    let bytes = tokio::fs::read(path)
        .await
        .with_context(|| format!("failed to read {}", path.display()))?;
    let payload = base64::engine::general_purpose::STANDARD.encode(bytes);
    data_url(&format!("data:{mime};base64,{payload}"))
}

fn mime(path: &Path) -> Result<&'static str> {
    match path.extension().and_then(|value| value.to_str()) {
        Some(value) if value.eq_ignore_ascii_case("png") => Ok("image/png"),
        Some(value) if ["jpg", "jpeg"].iter().any(|ext| value.eq_ignore_ascii_case(ext)) => {
            Ok("image/jpeg")
        }
        Some(value) if value.eq_ignore_ascii_case("gif") => Ok("image/gif"),
        Some(value) if value.eq_ignore_ascii_case("webp") => Ok("image/webp"),
        Some(value) if value.eq_ignore_ascii_case("bmp") => Ok("image/bmp"),
        Some(value) if value.eq_ignore_ascii_case("svg") => Ok("image/svg+xml"),
        _ => bail!("unknown image extension"),
    }
}
