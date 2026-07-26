use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::path::PathBuf;

pub(super) async fn save(
    encoded: &str,
    session_id: Option<&str>,
    call_id: Option<&str>,
) -> Result<PathBuf> {
    let bytes = STANDARD
        .decode(encoded.trim())
        .context("Images API returned invalid base64")?;
    let root = crate::config::Config::data_dir()
        .unwrap_or_else(|| PathBuf::from(".codetether-agent"))
        .join("generated_images")
        .join(sanitize(session_id.unwrap_or("unknown-session")));
    tokio::fs::create_dir_all(&root)
        .await
        .with_context(|| format!("failed to create `{}`", root.display()))?;
    let name = call_id
        .map(sanitize)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let path = root.join(format!("{name}.png"));
    tokio::fs::write(&path, bytes)
        .await
        .with_context(|| format!("failed to save `{}`", path.display()))?;
    Ok(path)
}

fn sanitize(value: &str) -> String {
    let value: String = value
        .chars()
        .map(|ch| match ch {
            ch if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') => ch,
            _ => '_',
        })
        .collect();
    (!value.is_empty())
        .then_some(value)
        .unwrap_or_else(|| "generated_image".into())
}