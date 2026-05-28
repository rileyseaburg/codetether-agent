//! Computer use request payload.

use super::ComputerUseAction;

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ComputerUseInput {
    pub action: ComputerUseAction,
    #[serde(default)]
    pub app: Option<String>,
    #[serde(default)]
    pub window_title_contains: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub button: Option<String>,
    #[serde(default)]
    pub modifiers: Vec<String>,
    #[serde(default)]
    pub scroll_amount: Option<i32>,
    #[serde(default)]
    pub hwnd: Option<i64>,
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
}
