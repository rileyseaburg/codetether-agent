//! Computer use action names.

#[derive(Copy, Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputerUseAction {
    Status,
    ListApps,
    RequestApp,
    Snapshot,
    WindowSnapshot,
    Click,
    RightClick,
    DoubleClick,
    Drag,
    MouseDown,
    MouseMove,
    MouseUp,
    TypeText,
    SetText,
    ClickClient,
    PressKey,
    Scroll,
    FocusViewport,
    BlenderSelectFrame,
    BringToFront,
    WaitMs,
    Stop,
}
