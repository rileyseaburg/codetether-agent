//! Computer use action names.

/// Requested computer-use operation.
///
/// # Examples
/// ```
/// use codetether_computer_contract::input::ComputerUseAction;
/// assert!(matches!(ComputerUseAction::Snapshot, ComputerUseAction::Snapshot));
/// ```
#[derive(Copy, Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputerUseAction {
    /// Request the status operation.
    Status,
    /// Request the list apps operation.
    ListApps,
    /// Request the request app operation.
    RequestApp,
    /// Request the snapshot operation.
    Snapshot,
    /// Request the window snapshot operation.
    WindowSnapshot,
    /// Request the ocr operation.
    Ocr,
    /// Request the ocr status operation.
    OcrStatus,
    /// Request the click operation.
    Click,
    /// Request the right click operation.
    RightClick,
    /// Request the double click operation.
    DoubleClick,
    /// Request the drag operation.
    Drag,
    /// Request the mouse down operation.
    MouseDown,
    /// Request the mouse move operation.
    MouseMove,
    /// Request the mouse up operation.
    MouseUp,
    /// Request the type text operation.
    TypeText,
    /// Request the set text operation.
    SetText,
    /// Request the click client operation.
    ClickClient,
    /// Request the press key operation.
    PressKey,
    /// Request the scroll operation.
    Scroll,
    /// Request the focus viewport operation.
    FocusViewport,
    /// Request the blender select frame operation.
    BlenderSelectFrame,
    /// Request the bring to front operation.
    BringToFront,
    /// Request the wait ms operation.
    WaitMs,
    /// Request the stop operation.
    Stop,
}
