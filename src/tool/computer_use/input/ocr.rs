//! Optional native OCR source and recognizer-language selection.

use std::path::PathBuf;

/// OCR options flattened into a `computer_use` request.
///
/// Without a path, OCR captures the specified HWND or current desktop.
/// Without a language, Windows chooses a recognizer from the user profile.
///
/// # Examples
/// ```rust
/// use codetether_agent::tool::computer_use::input::OcrInput;
/// let options = OcrInput::default();
/// assert!(options.path.is_none() && options.language.is_none());
/// ```
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OcrInput {
    /// Local image path; never downloaded or interpreted as a command.
    pub path: Option<PathBuf>,
    /// Optional BCP-47 tag, for example `en-US`.
    pub language: Option<String>,
}

impl OcrInput {
    /// Reject empty user-supplied selectors before WinRT calls.
    pub(crate) fn validate(&self, hwnd: Option<i64>) -> anyhow::Result<()> {
        anyhow::ensure!(hwnd != Some(0), "ocr hwnd must be nonzero");
        if let Some(path) = &self.path {
            anyhow::ensure!(!path.as_os_str().is_empty(), "ocr path must not be empty");
        }
        if let Some(language) = &self.language {
            anyhow::ensure!(
                !language.trim().is_empty(),
                "ocr language must not be empty"
            );
        }
        Ok(())
    }
}
