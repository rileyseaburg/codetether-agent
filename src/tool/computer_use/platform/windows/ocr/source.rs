//! OCR input bytes and provenance; image files never claim screen coordinates.

use anyhow::Context;
use serde_json::{Value, json};
use std::path::Path;
use std::io::Read;
use crate::tool::computer_use::capture_limits::{MAX_FILE_BYTES, file_bytes};

pub(super) struct Source {
    pub(super) bytes: Vec<u8>,
    pub(super) provenance: Value,
    pub(super) captured: bool,
}

pub(super) fn validate(path: Option<&Path>, hwnd: Option<i64>) -> anyhow::Result<()> {
    anyhow::ensure!(
        path.is_none() || hwnd.is_none(),
        "OCR path and hwnd are mutually exclusive; choose an image file or a window"
    );
    Ok(())
}

pub(super) fn load(path: Option<&Path>, hwnd: Option<i64>) -> anyhow::Result<Source> {
    if let Some(path) = path {
        let file = std::fs::File::open(path)
            .with_context(|| format!("Cannot read OCR image {}", path.display()))?;
        file_bytes(file.metadata()?.len())?;
        let mut bytes = Vec::new();
        file.take(MAX_FILE_BYTES + 1).read_to_end(&mut bytes)?;
        anyhow::ensure!(bytes.len() as u64 <= MAX_FILE_BYTES, "OCR file grew beyond input safety budget");
        return Ok(Source {
            bytes,
            provenance: json!({"kind": "file", "path": path, "screen_origin": null}),
            captured: false,
        });
    }
    super::capture::capture(hwnd)
}