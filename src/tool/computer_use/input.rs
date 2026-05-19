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
    /// Screenshot a specific window by hwnd
    WindowSnapshot,
    /// Click on coordinates or target
    Click,
    /// Right-click on coordinates
    RightClick,
    /// Double-click on coordinates
    DoubleClick,
    /// Drag from one point to another
    Drag,
    /// Type text using native input
    TypeText,
    /// Press key or hotkey
    PressKey,
    /// Scroll native window
    Scroll,
    /// Bring a window to the foreground
    BringToFront,
    /// Wait N milliseconds for UI to settle
    WaitMs,
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
    /// HWND of the target window (for bring_to_front, window_snapshot)
    #[serde(default)]
    pub hwnd: Option<i64>,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    /// End X coordinate for drag
    #[serde(default)]
    pub x2: Option<f64>,
    /// End Y coordinate for drag
    #[serde(default)]
    pub y2: Option<f64>,
    /// Milliseconds to wait (for wait_ms)
    #[serde(default)]
    pub ms: Option<u64>,
}
