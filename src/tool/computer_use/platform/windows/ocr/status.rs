//! Native OCR runtime probe; missing runtime and missing language are distinct.

use super::{apartment::Apartment, engine, identity, probe};
use crate::tool::{ToolResult, computer_use::response};
use serde_json::{Value, json};
use windows::Media::Ocr::OcrEngine;

pub(super) fn report() -> ToolResult {
    let payload = probe().unwrap_or_else(|error| json!({
        "backend": "windows_winrt_ocr", "runtime_available": false,
        "available": false, "error": format!("{error:#}"),
        "remediation": identity::SETUP_HINT
    }));
    response::success_result(payload)
}

fn probe() -> anyhow::Result<Value> {
    let _apartment = Apartment::enter()?;
    let package = identity::package_name()?;
    let languages = engine::languages()?;
    let maximum = OcrEngine::MaxImageDimension()?;
    let default_engine = engine::create(None);
    let default_language = default_engine
        .as_ref()
        .ok()
        .map(|engine| engine.RecognizerLanguage()?.LanguageTag())
        .transpose()?
        .map(|tag| tag.to_string());
    let probe_result = if package.is_some() {
        default_engine.as_ref().ok().map(probe::recognize)
    } else { None };
    let available = probe_result.as_ref().is_some_and(|result| result.is_ok());
    Ok(json!({
        "backend": "windows_winrt_ocr", "runtime_available": true,
        "available": available, "installed_recognizer_languages": languages,
        "package_identity_present": package.is_some(), "package_full_name": package,
        "recognition_probe_succeeded": available,
        "recognition_probe_error": probe_result.and_then(Result::err).map(|error| error.to_string()),
        "official_support_requires_package_identity": true,
        "max_image_dimension": maximum, "default_language": default_language,
        "recognizer_available": default_engine.is_ok(),
        "recognizer_error": default_engine.err().map(|error| error.to_string()),
        "remediation": if !available { Some(identity::SETUP_HINT) } else { None }
    }))
}