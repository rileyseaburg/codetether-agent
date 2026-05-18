use std::path::Path;

use crate::session::ImageAttachment;

pub(crate) fn attach(path: &Path) -> Result<ImageAttachment, String> {
    ensure_regular_file(path)?;
    let bytes = std::fs::read(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    let mime_type = super::image_mime::guess(path).ok_or_else(|| unsupported(path))?;
    tracing::info!(path = %path.display(), mime = %mime_type, size_bytes = bytes.len(), "Attached image file");
    Ok(ImageAttachment {
        data_url: super::image_data_url::encode(&mime_type, &bytes),
        mime_type: Some(mime_type),
    })
}

fn ensure_regular_file(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }
    if !path.is_file() {
        return Err(format!("Not a file: {}", path.display()));
    }
    Ok(())
}

fn unsupported(path: &Path) -> String {
    format!(
        "Unsupported image format: {}. Supported: {}",
        path.display(),
        super::image_mime::SUPPORTED_IMAGE_FORMATS
    )
}
