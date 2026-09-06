//! Computer use request payload.

use super::{ComputerUseAction, InputMode, OcrInput};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ComputerUseInput {
    pub action: ComputerUseAction,
    #[serde(default)]
    pub input_mode: InputMode,
    #[serde(flatten)]
    pub ocr: OcrInput,
    pub app: Option<String>,
    pub window_title_contains: Option<String>,
    pub text: Option<String>,
    pub key: Option<String>,
    pub button: Option<String>,
    #[serde(default)]
    pub modifiers: Vec<String>,
    pub scroll_amount: Option<i32>,
    pub hwnd: Option<i64>,
    #[serde(default)]
    pub viewport_child_hwnd: Option<i64>,
    #[serde(default)]
    pub client_area: bool,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    #[serde(default)]
    pub x2: Option<f64>,
    #[serde(default)]
    pub y2: Option<f64>,
    #[serde(default)]
    pub steps: Option<u32>,
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub ms: Option<u64>,
    #[serde(default)]
    pub object_name: Option<String>,
}
