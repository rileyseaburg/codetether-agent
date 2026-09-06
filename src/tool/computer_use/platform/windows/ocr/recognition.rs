//! Blocking native recognition orchestration and response assembly.

use super::{angle, bitmap, engine, output, source};
use crate::tool::{ToolResult, computer_use::{response, capture_preview}};
use serde_json::json;
use std::path::Path;
use windows::Media::Ocr::OcrEngine;

pub(super) fn run(
    path: Option<&Path>,
    language: Option<&str>,
    hwnd: Option<i64>,
) -> anyhow::Result<ToolResult> {
    let engine = engine::create(language)?;
    let source = source::load(path, hwnd)?;
    let maximum = OcrEngine::MaxImageDimension()?;
    let bitmap = bitmap::decode(&source.bytes, maximum)?;
    let result = engine.RecognizeAsync(&bitmap)?.join()?;
    let preview = source.captured.then(|| capture_preview::prepare(
        &source.bytes, bitmap.PixelWidth()? as u32, bitmap.PixelHeight()? as u32,
    )).transpose()?;
    let mut response = response::success_result(json!({
        "backend": "windows_winrt_ocr", "text": result.Text()?.to_string(),
        "language": engine.RecognizerLanguage()?.LanguageTag()?.to_string(),
        "requested_language": language, "lines": output::lines(&result)?,
        "width": bitmap.PixelWidth()?, "height": bitmap.PixelHeight()?,
        "max_image_dimension": maximum, "coordinate_space": "image_relative_pixels",
        "image_origin": {"x": 0, "y": 0}, "source": source.provenance,
        "orientation": "encoded_pixels_no_exif_rotation", "scaled": false,
        "text_angle_degrees": angle::degrees(result.TextAngle())?,
        "preview": preview.as_ref().map(|image| &image.mapping)
    }));
    if let Some(preview) = preview {
        response = response.with_metadata("image_data_url", preview.attachment);
    }
    Ok(response)
}