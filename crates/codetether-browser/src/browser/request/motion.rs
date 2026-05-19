//! Motion request payloads for browser commands.

use serde::Serialize;

/// Scroll command parameters.
///
/// `selector` targets a specific element when present. Otherwise the native
/// backend applies `x` and `y` deltas to the page scroll state.
#[derive(Debug, Clone, Serialize)]
pub struct ScrollRequest {
    /// Optional CSS selector for the element to scroll.
    pub selector: Option<String>,
    /// Optional horizontal scroll delta.
    pub x: Option<f64>,
    /// Optional vertical scroll delta.
    pub y: Option<f64>,
    /// Optional frame selector retained for API compatibility.
    pub frame_selector: Option<String>,
}
