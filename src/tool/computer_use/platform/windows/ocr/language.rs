//! Explicit BCP-47 validation before installed recognizer selection.

use anyhow::Context;
use windows::Globalization::Language;

pub(super) fn parse(tag: &str) -> anyhow::Result<Language> {
    anyhow::ensure!(
        !tag.is_empty()
            && tag
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-'),
        "Invalid OCR BCP-47 language tag: {tag:?}; use a tag such as en-US"
    );
    let tag_string = tag.into();
    anyhow::ensure!(
        Language::IsWellFormed(&tag_string)?,
        "Invalid OCR BCP-47 language tag: {tag:?}; use a tag such as en-US"
    );
    Language::CreateLanguage(&tag_string)
        .with_context(|| format!("Cannot create Windows language for OCR tag {tag:?}"))
}
