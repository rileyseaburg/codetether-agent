//! Computer use request payload.

use super::{ComputerUseAction, InputMode, OcrInput};

/// Parameters passed to native computer-use execution.
///
/// # Examples
/// ```
/// let input: codetether_computer_contract::input::ComputerUseInput =
///     serde_json::from_str(r#"{"action":"status"}"#).unwrap();
/// assert!(input.hwnd.is_none());
/// ```
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ComputerUseInput {
    /// Operation to perform.
    pub action: ComputerUseAction,
    #[serde(default)]
    /// Physical input or explicit shadow input.
    pub input_mode: InputMode,
    #[serde(flatten)]
    /// Flattened OCR source and language options.
    pub ocr: OcrInput,
    /// Application name for discovery or launch.
    pub app: Option<String>,
    /// Optional window-title filter.
    pub window_title_contains: Option<String>,
    /// Text to enter or set.
    pub text: Option<String>,
    /// Keyboard sequence to send.
    pub key: Option<String>,
    /// Mouse button name.
    pub button: Option<String>,
    #[serde(default)]
    /// Modifier keys held during an operation.
    pub modifiers: Vec<String>,
    /// Wheel delta.
    pub scroll_amount: Option<i32>,
    /// Target window handle.
    pub hwnd: Option<i64>,
    #[serde(default)]
    /// Optional child viewport handle.
    pub viewport_child_hwnd: Option<i64>,
    #[serde(default)]
    /// Whether coordinates are relative to the client area.
    pub client_area: bool,
    #[serde(default)]
    /// Starting horizontal coordinate.
    pub x: Option<f64>,
    #[serde(default)]
    /// Starting vertical coordinate.
    pub y: Option<f64>,
    #[serde(default)]
    /// Ending horizontal coordinate.
    pub x2: Option<f64>,
    #[serde(default)]
    /// Ending vertical coordinate.
    pub y2: Option<f64>,
    #[serde(default)]
    /// Number of intermediate pointer movements.
    pub steps: Option<u32>,
    #[serde(default)]
    /// Movement duration in milliseconds.
    pub duration_ms: Option<u64>,
    #[serde(default)]
    /// Wait duration in milliseconds.
    pub ms: Option<u64>,
    #[serde(default)]
    /// Object name for application-specific selection.
    pub object_name: Option<String>,
}
