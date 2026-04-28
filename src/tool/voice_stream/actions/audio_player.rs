//! HTML audio player helper.

use anyhow::{Context, Result, bail};
use tracing::info;

pub(crate) struct PlayerLaunch {
    pub html_path: std::path::PathBuf,
    pub browser_opened: bool,
}

/// Write a minimal HTML audio player and open the browser.
pub(crate) fn open(job_id: &str, output_url: &str) -> Result<PlayerLaunch> {
    let output_url = validate_url(output_url)?;
    let path = std::env::temp_dir().join(format!("voice_{}.html", safe_name(job_id)));
    std::fs::write(&path, html(job_id, output_url)).context("Failed to write HTML player")?;
    info!(job_id, output_url, "Opening voice audio player in browser");
    let browser_opened = open::that(&path).is_ok();
    Ok(PlayerLaunch {
        html_path: path,
        browser_opened,
    })
}

fn html(job_id: &str, output_url: &str) -> String {
    let title = escape_html(job_id);
    let source = escape_html(output_url);
    format!(
        r#"<!DOCTYPE html>
<html><head><title>Voice: {title}</title></head>
<body style="margin:0;display:flex;justify-content:center;align-items:center;height:100vh;background:#111">
<audio controls autoplay src="{source}" style="width:80%"></audio>
</body></html>"#
    )
}

fn validate_url(url: &str) -> Result<&str> {
    if url.starts_with("https://") || url.starts_with("http://") {
        return Ok(url);
    }
    bail!("Voice output URL must use http or https")
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn safe_name(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}
