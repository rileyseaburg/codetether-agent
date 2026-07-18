//! Structured input variants compatible with Codex collaboration calls.

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(super) enum InputItem {
    Text { text: String },
    Image { image_url: String },
    LocalImage { path: PathBuf },
    Audio { audio_url: String },
    LocalAudio { path: PathBuf },
    Skill { name: String, path: PathBuf },
    Mention { name: String, path: String },
}
