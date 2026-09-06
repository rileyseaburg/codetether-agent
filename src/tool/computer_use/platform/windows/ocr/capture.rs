//! Native screenshot acquisition with physical-screen provenance.

use super::{dpi::DpiContext, source::Source};
use crate::platform::windows::computer_use::{capture_screenshot, window};
use serde_json::json;

pub(super) fn capture(hwnd: Option<i64>) -> anyhow::Result<Source> {
    let _dpi = DpiContext::enter()?;
    let (bytes, width, height, x, y, kind) = if let Some(hwnd) = hwnd {
        let before = window::window_bounds(hwnd)?;
        let (bytes, width, height) = window::capture_window_png(hwnd)?;
        let after = window::window_bounds(hwnd)?;
        anyhow::ensure!(
            (before.left, before.top, before.right, before.bottom)
                == (after.left, after.top, after.right, after.bottom)
                && i64::from(after.right) - i64::from(after.left) == i64::from(width)
                && i64::from(after.bottom) - i64::from(after.top) == i64::from(height),
            "Window moved or resized during OCR capture; retry for reliable coordinates"
        );
        (bytes, width, height, before.left, before.top, "window")
    } else {
        let (bytes, width, height, x, y) = capture_screenshot()?;
        (bytes, width, height, x, y, "desktop")
    };
    Ok(Source {
        bytes,
        captured: true,
        provenance: json!({
            "kind": kind, "hwnd": hwnd, "width": width, "height": height,
            "screen_origin": {"x": x, "y": y},
            "screen_coordinate_space": "physical_screen_pixels",
            "window_region": hwnd.map(|_| "whole_window_including_nonclient_frame"),
            "coordinate_hint": "Add screen_origin to image-relative boxes for physical screen coordinates; window boxes are not client-relative.",
            "capture_note": "Native GDI capture reflects visible pixels; occluded/minimized windows may be incomplete."
        }),
    })
}
