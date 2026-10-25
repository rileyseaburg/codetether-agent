//! Versioned state compatible with the standalone Node image launcher.
use anyhow::{Result, bail};
use codetether_gemini_web_protocol::images::{Continuation, IMAGE_MODEL_ID};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct State {
    version: u8,
    model: String,
    conversation: Continuation,
}

pub(super) fn load(path: &Path) -> Result<Continuation> {
    let state_path = if path.is_dir() {
        path.join("conversation.json")
    } else if matches!(
        path.extension().and_then(|v| v.to_str()),
        Some("jpg" | "jpeg" | "png")
    ) {
        path.parent()
            .unwrap_or(Path::new("."))
            .join("conversation.json")
    } else {
        path.to_owned()
    };
    if std::fs::metadata(&state_path)?.len() > 16384 {
        bail!("Gemini refinement state exceeds size limit");
    }
    let state: State = serde_json::from_slice(&std::fs::read(state_path)?)
        .map_err(|_| anyhow::anyhow!("Invalid Gemini refinement state"))?;
    if state.version != 1 || state.model != IMAGE_MODEL_ID {
        bail!("Unsupported Gemini refinement state version or model");
    }
    state.conversation.validate()?;
    Ok(state.conversation)
}

pub(super) fn save(root: &Path, conversation: &Continuation) -> Result<PathBuf> {
    conversation.validate()?;
    let state = State {
        version: 1,
        model: IMAGE_MODEL_ID.into(),
        conversation: conversation.clone(),
    };
    let path = root.join("conversation.json");
    super::private_files::write(&path, &serde_json::to_vec_pretty(&state)?)?;
    Ok(path)
}
