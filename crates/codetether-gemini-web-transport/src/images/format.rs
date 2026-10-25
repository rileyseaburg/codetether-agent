use super::{GeneratedImage, jpeg};
use anyhow::{Result, anyhow, ensure};

pub(super) fn image(bytes: Vec<u8>, mime: &str) -> Result<GeneratedImage> {
    ensure!(
        !bytes.is_empty() && bytes.len() <= super::LIMIT,
        "image_size_invalid"
    );
    let (extension, dimensions) = match mime {
        "image/jpeg" => ("jpg", jpeg::dimensions(&bytes)),
        "image/png" => ("png", png(&bytes)),
        _ => return Err(anyhow!("image_mime_invalid")),
    };
    let (width, height) = dimensions.ok_or_else(|| anyhow!("image_content_invalid"))?;
    ensure!(
        width > 0 && height > 0 && u64::from(width) * u64::from(height) <= 100_000_000,
        "image_dimensions_invalid"
    );
    Ok(GeneratedImage {
        bytes,
        mime_type: mime.into(),
        extension: extension.into(),
        width,
        height,
    })
}
fn png(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 45
        || !bytes.starts_with(b"\x89PNG\r\n\x1a\n")
        || bytes[8..12] != 13u32.to_be_bytes()
        || &bytes[12..16] != b"IHDR"
        || !bytes.ends_with(b"\x00\x00\x00\x00IEND\xaeB`\x82")
    {
        return None;
    }
    Some((
        u32::from_be_bytes(bytes[16..20].try_into().ok()?),
        u32::from_be_bytes(bytes[20..24].try_into().ok()?),
    ))
}
