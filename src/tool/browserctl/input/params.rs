//! Browserctl input payload.

use serde::Deserialize;
use std::collections::HashMap;

/// Deserialized browserctl request parameters.
#[rustfmt::skip]
#[derive(Debug, Deserialize)]
pub(in crate::tool::browserctl) struct BrowserCtlInput {
    /// Requested browserctl action.
    pub action: super::BrowserCtlAction,
    /// Optional headless mode flag retained for API compatibility.
    #[serde(default)] pub headless: Option<bool>,
    /// Optional browser executable path retained for API compatibility.
    #[serde(default)] pub executable_path: Option<String>,
    /// Optional user data directory retained for API compatibility.
    #[serde(default)] pub user_data_dir: Option<String>,
    /// Optional websocket URL retained for API compatibility.
    #[serde(default)] pub ws_url: Option<String>,
    /// Target URL for navigation or network calls.
    #[serde(default)] pub url: Option<String>,
    /// Navigation wait strategy.
    #[serde(default)] pub wait_until: Option<String>,
    /// CSS selector for DOM actions.
    #[serde(default)] pub selector: Option<String>,
    /// Optional frame selector retained for API compatibility.
    #[serde(default)] pub frame_selector: Option<String>,
    /// Value used by fill and related actions.
    #[serde(default)] pub value: Option<String>,
    /// Text used by wait and click-text actions.
    #[serde(default)] pub text: Option<String>,
    /// Text expected to disappear during wait actions.
    #[serde(default)] pub text_gone: Option<String>,
    /// Optional delay in milliseconds.
    #[serde(default)] pub delay_ms: Option<u64>,
    /// Keyboard key name.
    #[serde(default)] pub key: Option<String>,
    /// JavaScript expression to evaluate.
    #[serde(default)] pub expression: Option<String>,
    /// URL substring used by wait actions.
    #[serde(default)] pub url_contains: Option<String>,
    /// Desired selector state for wait actions.
    #[serde(default)] pub state: Option<String>,
    /// Timeout in milliseconds.
    #[serde(default)] pub timeout_ms: Option<u64>,
    /// Output path for screenshot actions.
    #[serde(default)] pub path: Option<String>,
    /// Input file paths for upload actions.
    #[serde(default)] pub paths: Option<Vec<String>>,
    /// Whether screenshots should include the full page.
    #[serde(default)] pub full_page: Option<bool>,
    /// X coordinate or horizontal scroll delta.
    #[serde(default)] pub x: Option<f64>,
    /// Y coordinate or vertical scroll delta.
    #[serde(default)] pub y: Option<f64>,
    /// Tab index.
    #[serde(default)] pub index: Option<usize>,
    /// Whether text matching should be exact.
    #[serde(default)] pub exact: Option<bool>,
    /// HTTP method.
    #[serde(default)] pub method: Option<String>,
    /// HTTP headers.
    #[serde(default)] pub headers: Option<HashMap<String, String>>,
    /// HTTP request body.
    #[serde(default)] pub body: Option<String>,
    /// Fetch credentials mode.
    #[serde(default)] pub credentials: Option<String>,
    /// Result limit for log-style actions.
    #[serde(default)] pub limit: Option<usize>,
    /// Dot path for extracting axios response data.
    #[serde(default)] pub axios_path: Option<String>,
    /// JSON body for axios-style requests.
    #[serde(default)] pub json_body: Option<serde_json::Value>,
    /// JSON patch merged into replay request bodies.
    #[serde(default)] pub body_patch: Option<serde_json::Value>,
    /// Whether axios-style requests include credentials.
    #[serde(default)] pub with_credentials: Option<bool>,
}
