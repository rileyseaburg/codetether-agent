//! Browser command messages accepted by a [`BrowserSession`](crate::browser::BrowserSession).
//!
//! The command enum keeps tool-facing request parsing separate from the
//! session backend that executes each operation.

use super::request::{
    AxiosRequest, ClickTextRequest, CloseTabRequest, DiagnoseRequest, EvalRequest, FetchRequest,
    FillRequest, KeyPressRequest, NavigationRequest, NetworkLogRequest, NewTabRequest,
    ReplayRequest, ScopeRequest, ScreenshotRequest, ScrollRequest, SelectorRequest, StartRequest,
    TabSelectRequest, ToggleRequest, TypeRequest, UploadRequest, WaitRequest, XhrRequest,
};

/// Operation that can be executed against a browser session.
///
/// Variants carry the request payload needed by the native TetherScript backend
/// and by callers that route browserctl actions through the shared browser
/// service.
pub enum BrowserCommand {
    /// Report session health.
    Health,
    /// Start a browser session.
    Start(StartRequest),
    /// Stop the active browser session.
    Stop,
    /// Return a structured page snapshot.
    Snapshot,
    /// Navigate the current tab.
    Goto(NavigationRequest),
    /// Move back in the current tab history.
    Back,
    /// Reload the current tab.
    Reload,
    /// Wait for a selector or timeout.
    Wait(WaitRequest),
    /// Click an element by selector.
    Click(SelectorRequest),
    /// Move the pointer over an element by selector.
    Hover(SelectorRequest),
    /// Focus an element by selector.
    Focus(SelectorRequest),
    /// Blur an element by selector.
    Blur(SelectorRequest),
    /// Scroll the current page or selected element.
    Scroll(ScrollRequest),
    /// Upload files through an input element.
    Upload(UploadRequest),
    /// Fill an input-like element.
    Fill(FillRequest),
    /// Type text into the active page.
    Type(TypeRequest),
    /// Press a keyboard key.
    Press(KeyPressRequest),
    /// Read text content.
    Text(ScopeRequest),
    /// Read HTML content.
    Html(ScopeRequest),
    /// Evaluate JavaScript in the current page.
    Eval(EvalRequest),
    /// Click by visible text.
    ClickText(ClickTextRequest),
    /// Fill an input through the native path.
    FillNative(FillRequest),
    /// Toggle a checkbox-like control.
    Toggle(ToggleRequest),
    /// Capture a screenshot.
    Screenshot(ScreenshotRequest),
    /// Click at page coordinates.
    MouseClick(super::request::PointerClick),
    /// Type raw keyboard text.
    KeyboardType(super::request::KeyboardTypeRequest),
    /// Press a named keyboard key.
    KeyboardPress(super::request::KeyboardPressRequest),
    /// List open tabs.
    Tabs,
    /// Select a tab by index.
    TabsSelect(TabSelectRequest),
    /// Open a new tab.
    TabsNew(NewTabRequest),
    /// Close a tab.
    TabsClose(CloseTabRequest),
    /// Return recorded network events.
    NetworkLog(NetworkLogRequest),
    /// Issue a fetch-style HTTP request.
    Fetch(FetchRequest),
    /// Issue an axios-style HTTP request.
    Axios(AxiosRequest),
    /// Issue an XHR-style HTTP request.
    Xhr(XhrRequest),
    /// Replay a captured HTTP request.
    Replay(ReplayRequest),
    /// Return backend diagnostics.
    Diagnose(DiagnoseRequest),
}
