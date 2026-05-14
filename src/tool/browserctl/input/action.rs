//! Supported browserctl action names.

use serde::Deserialize;

/// Browserctl action parsed from the `action` input field.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(in crate::tool::browserctl) enum BrowserCtlAction {
    /// Return browser session health.
    Health,
    /// Start a browser session.
    Start,
    /// Stop the browser session.
    Stop,
    /// Return a structured snapshot.
    Snapshot,
    /// Navigate to a URL.
    Goto,
    /// Navigate backward.
    Back,
    /// Click an element.
    Click,
    /// Hover an element.
    Hover,
    /// Focus an element.
    Focus,
    /// Blur an element.
    Blur,
    /// Scroll the page or an element.
    Scroll,
    /// Upload files.
    Upload,
    /// Fill an element.
    Fill,
    /// Type text.
    Type,
    /// Press a key.
    Press,
    /// Read text.
    Text,
    /// Read HTML.
    Html,
    /// Evaluate JavaScript.
    Eval,
    /// Click visible text.
    ClickText,
    /// Fill through the native path.
    FillNative,
    /// Toggle a control.
    Toggle,
    /// Capture a screenshot.
    Screenshot,
    /// Click at coordinates.
    MouseClick,
    /// Type raw keyboard text.
    KeyboardType,
    /// Press a named key.
    KeyboardPress,
    /// Reload the page.
    Reload,
    /// Wait for a condition.
    Wait,
    /// List tabs.
    Tabs,
    /// Select a tab.
    TabsSelect,
    /// Open a tab.
    TabsNew,
    /// Close a tab.
    TabsClose,
    /// Detect browser backend availability.
    Detect,
    /// Read network logs.
    NetworkLog,
    /// Perform a fetch-style request.
    Fetch,
    /// Perform an axios-style request.
    Axios,
    /// Perform an XHR-style request.
    Xhr,
    /// Replay a captured request.
    Replay,
    /// Return backend diagnostics.
    Diagnose,
}
