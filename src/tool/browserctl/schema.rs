//! JSON Schema advertised by the browserctl tool.

use serde_json::{Value, json};

pub(super) fn parameters_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": {"type": "string", "enum": ["health","detect","start","stop","snapshot","goto","click","hover","focus","blur","scroll","upload","fill","type","press","text","html","eval","click_text","fill_native","toggle","screenshot","mouse_click","keyboard_type","keyboard_press","reload","tabs","tabs_select","tabs_new","tabs_close","back","wait","network_log","fetch","axios","xhr","replay","diagnose"], "description": "Browserctl action to execute. `network_log` returns recent fetch+XHR in the active tab; `fetch` replays raw HTTP from inside the page; `axios` replays via the page's own axios client (inherits interceptors/auth/baseURL — use this when `fetch` returns 'Failed to fetch'); `xhr` replays as a raw XMLHttpRequest (use this when the app's successful request in `network_log` shows `kind: xhr` and fetch/axios fail — it matches the app's Sec-Fetch-* and cookie semantics byte-for-byte); `replay` re-fires a captured network_log entry, inheriting its Authorization and other request headers, with optional `body_patch` (JSON deep-merge) or `body` (full override) — use this after the user performs one real save in the UI, to re-save with edits without reconstructing the request; `diagnose` dumps service workers, axios instances, CSP, and network log summary."},
            "headless": {"type": "boolean", "description": "For start: launch headless browser (default true)"},
            "executable_path": {"type": "string", "description": "For start: optional browser binary path"},
            "user_data_dir": {"type": "string", "description": "For start in launch mode: optional browser profile directory"},
            "ws_url": {"type": "string", "description": "For start in connect mode: DevTools websocket URL for an existing browser"},
            "url": {"type": "string", "description": "For goto or tabs_new: target URL"},
            "wait_until": {"type": "string", "enum": ["commit","domcontentloaded","load"], "description": "For goto: wait strategy. `commit` = return as soon as the URL is committed (fastest). `domcontentloaded` = wait until the DOM is parsed (default, good for most scraping). `load` = wait for the full load event incl. all subresources (slowest, most like a real user waiting)."},
            "selector": {"type": "string", "description": "CSS selector for selector-based actions. Pierces shadow DOM automatically; use the `>>>` combinator to drill through specific shadow boundaries (e.g. `colab-composer-rich-text-field >>> .inputarea`)."},
            "frame_selector": {"type": "string", "description": "Optional iframe selector for frame-scoped actions"},
            "value": {"type": "string", "description": "Text value for fill/fill_native"},
            "text": {"type": "string", "description": "Visible text or typed text depending on action"},
            "text_gone": {"type": "string", "description": "For wait: text that must disappear"},
            "delay_ms": {"type": "integer", "description": "For type: per-character delay in ms"},
            "key": {"type": "string", "description": "For press/keyboard_press: key to press, e.g. Enter"},
            "expression": {"type": "string", "description": "For eval: page expression to evaluate"},
            "url_contains": {"type": "string", "description": "For wait: wait until the page URL contains this substring"},
            "state": {"type": "string", "description": "For wait: selector/text wait state, default visible"},
            "timeout_ms": {"type": "integer", "description": "For click_text/toggle/wait: timeout in ms. For eval: max time the async expression may run before returning a structured timeout error (default 30000)."},
            "path": {"type": "string", "description": "For screenshot: destination path. For upload: single file path shorthand"},
            "paths": {"type": "array", "items": {"type": "string"}, "description": "For upload: one or more file paths to assign to an <input type=file>"},
            "full_page": {"type": "boolean", "description": "For screenshot: capture full page (default true)"},
            "x": {"type": "number", "description": "For mouse_click: X coordinate"},
            "y": {"type": "number", "description": "For mouse_click: Y coordinate"},
            "index": {"type": "integer", "description": "For click_text nth match or tabs_select/tabs_close: tab index"},
            "exact": {"type": "boolean", "description": "For click_text: exact text match (default true)"},
            "method": {"type": "string", "description": "For fetch/network_log: HTTP method (GET/POST/PUT/PATCH/DELETE). Defaults to GET for fetch, any-method for network_log."},
            "headers": {"type": "object", "additionalProperties": {"type": "string"}, "description": "For fetch: request headers as a JSON object. Copy Authorization from network_log output to replay authenticated requests."},
            "body": {"type": "string", "description": "For fetch: request body as a string (typically JSON.stringify(payload))."},
            "credentials": {"type": "string", "enum": ["omit", "same-origin", "include"], "description": "For fetch: fetch credentials mode. Defaults to 'include' so cookies travel with the request."},
            "limit": {"type": "integer", "description": "For network_log: max number of recent entries to return."},
            "axios_path": {"type": "string", "description": "For axios: optional dotted path to the axios instance on window (e.g. 'window.__APP__.api'). Omit to auto-discover."},
            "json_body": {"description": "For axios: request body as a parsed JSON value (object/array/number/etc). Preferred over `body` for axios since it preserves types. If both are set, `json_body` wins."},
            "body_patch": {"description": "For replay: JSON object that is deep-merged into the captured request body when the body parses as JSON. Keys present in the patch overwrite keys in the captured body; keys absent from the patch are preserved. Ignored when `body` (full override) is set or when the captured body is not JSON."},
            "with_credentials": {"type": "boolean", "description": "For xhr/replay: set `xhr.withCredentials = true` (default true) so cookies and Authorization travel cross-origin."}
        },
        "required": ["action"],
        "examples": [{"action": "health"}, {"action": "start", "ws_url": "ws://localhost:9222/devtools/browser/session-id"}, {"action": "start", "headless": true, "user_data_dir": "/tmp/codetether-browser"}, {"action": "goto", "url": "https://github.com"}, {"action": "back"}, {"action": "wait", "text": "Environment is ready", "timeout_ms": 15000}, {"action": "eval", "expression": "({ title: document.title, url: location.href })"}, {"action": "upload", "selector": "input[type=file]", "paths": ["/tmp/submission.zip"]}, {"action": "fill_native", "selector": "#email", "value": "user@example.com"}, {"action": "toggle", "selector": "#rating", "text": "1"}, {"action": "screenshot", "path": "/tmp/page.png", "full_page": true}, {"action": "network_log", "url_contains": "update-task", "limit": 5}, {"action": "fetch", "method": "PUT", "url": "https://api.example.com/v1/items/42", "headers": {"Authorization": "Bearer eyJ...", "Content-Type": "application/json"}, "body": "{\"value\":123}"}, {"action": "diagnose"}, {"action": "axios", "method": "PUT", "url": "/api/v1/items/42", "json_body": {"value": 123}}, {"action": "xhr", "method": "PUT", "url": "https://api.example.com/update-task/27", "headers": {"Content-Type": "application/json", "Authorization": "Bearer eyJ..."}, "body": "{\"value\":123}"}, {"action": "replay", "url_contains": "/update-task/27", "method": "PUT", "body_patch": {"domain": "production"}}]
    })
}
