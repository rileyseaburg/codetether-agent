//! JSON Schema advertised by the browserctl tool.

use serde_json::{Value, json};

pub(super) fn parameters_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {
                "type": "string",
                "enum": [
                    "health","start","stop","snapshot","console","goto","click","fill","type",
                    "press","text","html","eval","console_eval","click_text","fill_native",
                    "toggle","screenshot","mouse_click","keyboard_type","keyboard_press",
                    "reload","tabs","tabs_select","tabs_new","tabs_close","back","wait"
                ],
                "description": "Browserctl action to execute"
            },
            "base_url": {"type": "string", "description": "Override browserctl base URL (default: BROWSERCTL_BASE or http://127.0.0.1:4477)"},
            "token": {"type": "string", "description": "Bearer token override (default: BROWSERCTL_TOKEN env)"},
            "headless": {"type": "boolean", "description": "For start: launch headless browser (default true)"},
            "executable_path": {"type": "string", "description": "For start: optional browser binary path"},
            "url": {"type": "string", "description": "For goto or tabs_new: target URL"},
            "wait_until": {"type": "string", "description": "For goto: wait strategy, default domcontentloaded"},
            "selector": {"type": "string", "description": "CSS selector for selector-based actions"},
            "frame_selector": {"type": "string", "description": "Optional iframe selector for frame-scoped actions"},
            "value": {"type": "string", "description": "Text value for fill/fill_native"},
            "text": {"type": "string", "description": "Visible text or typed text depending on action"},
            "text_gone": {"type": "string", "description": "For wait: text that must disappear"},
            "delay_ms": {"type": "integer", "description": "For type: per-character delay in ms"},
            "key": {"type": "string", "description": "For press/keyboard_press: key to press, e.g. Enter"},
            "expression": {"type": "string", "description": "For eval: page expression to evaluate"},
            "script": {"type": "string", "description": "For console_eval: async page-side script"},
            "url_contains": {"type": "string", "description": "For wait: wait until the page URL contains this substring"},
            "state": {"type": "string", "description": "For wait: selector/text wait state, default visible"},
            "timeout_ms": {"type": "integer", "description": "For click_text/toggle: timeout in ms"},
            "path": {"type": "string", "description": "For screenshot: destination path"},
            "full_page": {"type": "boolean", "description": "For screenshot: capture full page (default true)"},
            "x": {"type": "number", "description": "For mouse_click: X coordinate"},
            "y": {"type": "number", "description": "For mouse_click: Y coordinate"},
            "index": {"type": "integer", "description": "For click_text nth match or tabs_select/tabs_close: tab index"},
            "exact": {"type": "boolean", "description": "For click_text: exact text match (default true)"}
        },
        "required": ["action"],
        "examples": [
            {"action": "health"},
            {"action": "goto", "url": "https://github.com"},
            {"action": "back"},
            {"action": "wait", "text": "Environment is ready", "timeout_ms": 15000},
            {"action": "console_eval", "script": "return { title: document.title, url: location.href };"},
            {"action": "fill_native", "selector": "#email", "value": "user@example.com"},
            {"action": "toggle", "selector": "#rating", "text": "1"},
            {"action": "screenshot", "path": "/tmp/page.png", "full_page": true}
        ]
    })
}
