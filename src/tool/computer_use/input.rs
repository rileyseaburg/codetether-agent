//! Input types for the computer_use tool.

#[derive(Copy, Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputerUseAction {
    /// Check OS support and permission status
    Status,
    /// List visible controllable apps/windows
    ListApps,
    /// Request approval for target app
    RequestApp,
    /// Capture screenshot/accessibility snapshot
    Snapshot,
    /// Click on coordinates or target
    Click,
    /// Type text using native input
    TypeText,
    /// Press key or hotkey
    PressKey,
    /// Scroll native window
    Scroll,
    /// Stop active control
    Stop,
}

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
    pub scroll_amount: Option<i32>,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
}
