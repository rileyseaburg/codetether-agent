//! HTML audio player helper — temp file + browser launch.

use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use tracing::info;

pub(crate) struct Launch {
    pub(crate) html_path: PathBuf,
    pub(crate) browser_opened: bool,
}

pub(crate) fn open(job_id: &str, output_url: &str) -> Result<Launch> {
    let output_url = validate_url(output_url)?;
    let path = player_path(job_id);
    std::fs::write(&path, player_html(job_id, output_url))
        .context("Failed to write HTML player")?;
    info!(job_id, output_url, "Opening voice audio player in browser");
    let browser_opened = open::that(&path).is_ok();
    Ok(Launch {
        html_path: path,
        browser_opened,
    })
}

fn player_path(job_id: &str) -> PathBuf {
    std::env::temp_dir().join(format!("voice_{}.html", safe_name(job_id)))
}

fn player_html(job_id: &str, output_url: &str) -> String {
    let title = escape_html(job_id);
    let source = escape_html(output_url);
    format!(
        r#"<!DOCTYPE html><html><head><title>Voice: {title}</title></head>
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
        .replace('\'', "&#39;")
}

fn safe_name(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}
