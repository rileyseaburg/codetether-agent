//! HTML audio player helper — temp file + open in browser.

use anyhow::{Context, Result, bail};
use tracing::info;

/// Write a minimal HTML audio player to a temp file and open the
/// default browser.
pub(crate) fn open(job_id: &str, output_url: &str) -> Result<()> {
    let output_url = validate_url(output_url)?;
    let title = escape_html(job_id);
    let source = escape_html(output_url);
    let html = format!(
        r#"<!DOCTYPE html>
<html><head><title>Voice: {title}</title></head>
<body style="margin:0;display:flex;justify-content:center;align-items:center;height:100vh;background:#111">
<audio controls autoplay src="{source}" style="width:80%"></audio>
</body></html>"#
    );

    let path = std::env::temp_dir().join(format!("voice_{}.html", safe_name(job_id)));
    std::fs::write(&path, html).context("Failed to write HTML player")?;

    info!(job_id, output_url, "Opening voice audio player in browser");
    open::that(path).context("Failed to open browser")?;
    Ok(())
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
