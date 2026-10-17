//! HTML audio player helper — temp file + browser launch.

use super::player_document::{player_html, player_path, validate_url};
use anyhow::{Context, Result};
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
    info!(target: "codetether_agent::tool::voice_stream::actions::audio_player", job_id, output_url, "Opening voice audio player in browser");
    Ok(Launch {
        html_path: path,
        browser_opened: open::that(player_path(job_id)).is_ok(),
    })
}
