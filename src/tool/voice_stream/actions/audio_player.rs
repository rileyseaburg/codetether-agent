//! HTML audio player helper — temp file + open in browser.

use anyhow::{Context, Result};
use tracing::info;

/// Write a minimal HTML audio player to a temp file and open the
/// default browser. The temp dir is leaked so the file survives
/// long enough for the browser to load it.
pub(crate) fn open(job_id: &str, output_url: &str) -> Result<()> {
    let html = format!(
        r#"<!DOCTYPE html>
<html><head><title>Voice: {job_id}</title></head>
<body style="margin:0;display:flex;justify-content:center;align-items:center;height:100vh;background:#111">
<audio controls autoplay src="{output_url}" style="width:80%"></audio>
</body></html>"#
    );

    let dir = tempfile::tempdir().context("Failed to create temp dir")?;
    let path = dir.path().join(format!("voice_{job_id}.html"));
    std::fs::write(&path, html).context("Failed to write HTML player")?;

    // Leak the temp dir so the file persists for the browser.
    let _ = dir.into_path();

    info!(job_id, output_url, "Opening voice audio player in browser");
    open::that(path).context("Failed to open browser")?;
    Ok(())
}
