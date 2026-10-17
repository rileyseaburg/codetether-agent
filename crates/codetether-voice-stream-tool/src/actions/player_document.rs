//! Safe HTML rendering, audio URL validation and temporary player paths.
use anyhow::{Result, bail};
use std::path::PathBuf;

pub(super) fn player_path(job_id: &str) -> PathBuf {
    std::env::temp_dir().join(format!("voice_{}.html", safe_name(job_id)))
}

pub(super) fn player_html(job_id: &str, output_url: &str) -> String {
    let title = escape_html(job_id);
    let source = escape_html(output_url);
    format!(
        r#"<!DOCTYPE html><html><head><title>Voice: {title}</title></head><body style="margin:0;display:flex;justify-content:center;align-items:center;height:100vh;background:#111"><audio controls autoplay src="{source}" style="width:80%"></audio></body></html>"#
    )
}

pub(super) fn validate_url(url: &str) -> Result<&str> {
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
