//! JSON Schema advertised by the computer_use tool.

use serde_json::{Value, json};

mod schema_examples;
#[cfg(test)]
#[path = "schema_tests.rs"]
mod tests;

pub(super) fn parameters_schema() -> Value {
    json!({
        "type": "object",
        "description": "Native Windows desktop control and OCR. Physical input is the default; shadow input queues HWND-targeted messages without moving the real cursor or changing focus.",
        "properties": {
            "action": {
                "type": "string",
                "enum": [
                    "status", "list_apps", "request_app",
                    "snapshot", "window_snapshot", "ocr", "ocr_status",
                    "click", "right_click", "double_click", "drag",
                    "mouse_down", "mouse_move", "mouse_up",
                    "type_text", "press_key", "scroll",
                    "focus_viewport", "blender_select_frame",
                    "bring_to_front", "wait_ms", "stop"
                ],
                "description": "Action to execute. Snapshot returns physical screen pixel bounds and real cursor position. Use bring_to_front before interacting."
            },
            "app": {"type": "string", "description": "App name for request_app."},
            "input_mode": {"type": "string", "enum": ["physical", "shadow"], "default": "physical", "description": "Shadow requires hwnd, never falls back to physical input, and reports queued messages rather than confirmed application effects. Not all applications accept background messages."},
            "path": {"type": "string", "description": "For ocr: local image path. Omit to capture hwnd or the desktop. No Python/Tesseract required."},
            "language": {"type": "string", "description": "For ocr: optional installed Windows OCR language tag, e.g. en-US. Use ocr_status to inspect availability."},
            "hwnd": {"type": "integer", "description": "Window handle from list_apps. Required for physical type_text/press_key and must already be foreground. Mouse x/y are window-relative with hwnd. For resized snapshot previews, apply the reported preview scales first."},
            "viewport_child_hwnd": {"type": "integer", "description": "Optional Blender viewport child HWND to focus before viewport helper actions."},
            "client_area": {"type": "boolean", "description": "With hwnd, interpret x/y relative to the window client area instead of the outer frame. Useful for Blender child viewports."},
            "text": {"type": "string", "description": "Text for type_text."},
            "key": {"type": "string", "description": "SendKeys for press_key: ^c, %{TAB}, ENTER."},
            "button": {"type": "string", "enum": ["left", "middle", "right"], "description": "Mouse button for drag; default left. Use middle for Blender viewport orbit."},
            "modifiers": {"type": "array", "items": {"type": "string", "enum": ["shift", "ctrl", "alt"]}, "description": "Modifier keys held during mouse actions. Use shift+middle drag to pan Blender."},
            "scroll_amount": {"type": "integer", "description": "Wheel delta; negative=down. Provide x/y to move cursor before scrolling."},
            "x": {"type": "number", "description": "Physical screen pixel X, or window-relative X when hwnd is provided."},
            "y": {"type": "number", "description": "Physical screen pixel Y, or window-relative Y when hwnd is provided."},
            "x2": {"type": "number", "description": "Physical screen pixel end X, or window-relative end X when hwnd is provided."},
            "y2": {"type": "number", "description": "Physical screen pixel end Y, or window-relative end Y when hwnd is provided."},
            "steps": {"type": "integer", "minimum": 1, "maximum": 240, "description": "Intermediate cursor moves for drag; use 8-20 for Blender viewport gestures."},
            "duration_ms": {"type": "integer", "minimum": 0, "maximum": 30000, "description": "Total drag movement duration in milliseconds."},
            "ms": {"type": "integer", "description": "Ms to wait for wait_ms."},
            "object_name": {"type": "string", "description": "Object name to select in Blender via UI search."}
        },
        "required": ["action"],
        "examples": schema_examples::examples()
    })
}
