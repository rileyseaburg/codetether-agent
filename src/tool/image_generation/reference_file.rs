use anyhow::{Context, Result, bail};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::path::Path;

pub(super) async fn load(path: &Path) -> Result<String> {
    let bytes = tokio::fs::read(path)
        .await
        .with_context(|| format!("unable to read referenced image `{}`", path.display()))?;
    let mime = mime(path)?;
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(bytes)))
}

fn mime(path: &Path) -> Result<&'static str> {
    match path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("png") => Ok("image/png"),
        Some("jpg" | "jpeg") => Ok("image/jpeg"),
        Some("webp") => Ok("image/webp"),
        _ => bail!("unsupported image type `{}`", path.display()),
    }
}
