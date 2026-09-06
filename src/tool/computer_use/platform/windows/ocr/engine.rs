//! Installed language discovery and explicit native recognizer selection.

use anyhow::Context;
use serde_json::{Value, json};
use windows::Media::Ocr::OcrEngine;

pub(super) const INSTALL_HINT: &str = "Run the CodeTether Windows installer to provision OCR language support automatically. No Python or Tesseract setup is needed.";

pub(super) fn languages() -> anyhow::Result<Vec<Value>> {
    OcrEngine::AvailableRecognizerLanguages()?
        .into_iter()
        .map(|language| {
            Ok(json!({"language_tag": language.LanguageTag()?.to_string(),
            "display_name": language.DisplayName()?.to_string()}))
        })
        .collect()
}

pub(super) fn create(tag: Option<&str>) -> anyhow::Result<OcrEngine> {
    if let Some(tag) = tag {
        let language = super::language::parse(tag)?;
        anyhow::ensure!(
            OcrEngine::IsLanguageSupported(&language)?,
            "Windows OCR language {tag} is not installed/supported. {INSTALL_HINT}"
        );
        return OcrEngine::TryCreateFromLanguage(&language).with_context(|| {
            format!("Cannot create Windows OCR recognizer for {tag}. {INSTALL_HINT}")
        });
    }
    if let Ok(engine) = OcrEngine::TryCreateFromUserProfileLanguages() {
        return Ok(engine);
    }
    for language in OcrEngine::AvailableRecognizerLanguages()? {
        if let Ok(engine) = OcrEngine::TryCreateFromLanguage(&language) {
            return Ok(engine);
        }
    }
    anyhow::bail!("No usable Windows OCR recognizer. {INSTALL_HINT}")
}