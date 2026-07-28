# Browser Capability API

TetherScript has a browser authority (`browser_cap::BrowserAuthority`) designed
as the seam between scripts and native tetherscript browser infrastructure. It
is not an adapter to another browser engine. The authority posts compact action
envelopes to a tetherscript browser host and records an in-process trace for
audit and replay.

## Granting

CLI example:

```bash
tetherscript run --interp \
  --grant-browser http://127.0.0.1:41707/browser \
  --browser-origin http://localhost:5173 \
  --browser-scope all \
  examples/browser_agentic_debug.tether
```

Default scopes, when `--browser-scope` is omitted, are enough for Milestone 1: navigate, interact, DOM inspect, console inspect, network inspect, screenshot.

Supported scopes:

- `browser.navigate`
- `browser.interact`
- `browser.inspect.dom`
- `browser.inspect.network`
- `browser.inspect.console`
- `browser.inspect.storage`
- `browser.inspect.react`
- `browser.mutate.storage`
- `browser.replay.network`
- `browser.screenshot`
- `browser.visual`

Authorities can be narrowed with a map containing `origins`, `scopes`, `path_prefix`, `storage_scope`, and `human_approval`.

## Native Host Contract

For a method like `browser.goto(url)`, TetherScript sends:

```http
POST /browser
Content-Type: application/json

{"action":"goto","url":"http://localhost:5173"}
```

The host may return either a raw JSON value or:

```json
{"ok": true, "value": {}}
```

Errors use:

```json
{"ok": false, "error": "selector not found"}
```

CodeTether-style tool results are also accepted. When a successful response has
`success: true` and string `output`, TetherScript parses `output` as JSON when
possible and otherwise returns it as a string.

Live native-host smoke test:

```bash
TETHERSCRIPT_BROWSERCTL_ENDPOINT=http://127.0.0.1:41707/browser \
  cargo test --test browser_cap_live
```

The test starts a local fixture page, then drives the configured tetherscript
browser endpoint through start, goto, wait, eval, snapshot, screenshot, and
stop.

## MVP methods

Navigation/interaction:
`goto`, `reload`, `back`, `click`, `click_text`, `type`, `press`,
`hover`, `focus`, `blur`, `scroll`, `wait_for_selector`, `wait_for_text`,
`wait_for_url`, `wait_for_network_idle`.

The wait helpers are tetherscript convenience methods over the host
`wait` action:

- `wait_for_selector(selector, timeout_ms?)` sends `{"action":"wait","selector":...,"state":"visible","timeout_ms":...}`.
- `wait_for_text(text, timeout_ms?)` sends `{"action":"wait","text":...,"timeout_ms":...}`.
- `wait_for_url(url_substring, timeout_ms?)` sends `{"action":"wait","url_contains":...,"timeout_ms":...}`.

`wait_for_network_idle`, `compare_screenshots`, and `visual_diff` are reserved
language-level methods. The current native host rejects them before network I/O
until matching actions exist.

`forward` is also reserved until the native host exposes a forward history
action.

`scroll` accepts `scroll(selector)`, `scroll(x, y)`, or
`scroll(selector, x, y)` and sends a `scroll` action envelope with the matching
fields.

Snapshots:
`screenshot`, `screenshot_element`, `dom_snapshot`, `page_snapshot`.

Diagnostics:
`console_logs`, `console_errors`, `unhandled_rejections`, `runtime_exceptions`, `source_mapped_stack_traces`.

Native page diagnostics:
`BrowserPage::production_debug_report()` returns a single structured report for
agent-side bundled UI debugging: console errors, page errors, failed requests,
HAR-style network entries, source-map registration status, source-mapped
page-error locations and generated stack frames, framework markers, React roots,
hydration warnings, classified runtime exceptions, and native visual element
evidence with selector candidates, computed styles, visibility, and layout
bounds. This is the native
full-parity track's production-debug surface; it does not depend on an external
browser engine or remote-control driver.

Native page actions are expected to exercise production form code paths:
`fill` updates live form values and dispatches input/change, `click` uses a
user-like pointer/mouse/focus/click sequence, prevented submits do not navigate,
and Enter on an input submits its containing form.

Network:
`network_log`, `network_har`, `failed_requests`, `request`, `response`, `replay_request`, `wait_for_request`, `wait_for_response`.

Storage:
`cookies`, `local_storage`, `session_storage`, `indexed_db_summary`, `set_cookie`, `set_local_storage`, `clear_storage`.

React/framework hooks:
Use string method syntax for dotted method names, e.g. `browser."react.detect"()?`, `browser."react.component_for_selector"("#root")?`, plus `frameworks()` for Next/Vite/Redux/Zustand/React Query detection returned by the host.

Trace/export:
`trace`, `export_trace_json`, `export_har`, `agent_summary`, `minimal_reproduction_script`.

Raw action:
`raw(action, params)` sends an explicit host action envelope after the same
scope and origin checks as high-level methods. This is an FFI-style escape hatch;
the stable API remains the language-level browser methods above. Raw action
names are restricted to the native host's advertised action enum.

## Agent assertions

Runtime assertion helpers return `Result` values suitable for `?` propagation:

- `assert_selector(browser, selector)`
- `assert_text(browser, text)`
- `assert_no_console_errors(browser)`
- `assert_no_failed_requests(browser)`
- `assert_visible(browser, selector)`
- `assert_enabled(browser, selector)`
- `assert_route(browser, path_or_url_substring)`
- `assert_react_component(browser, name)`

`assert_screenshot_matches(browser, name)` is reserved until the native host has
a baseline-image comparison action. Today it returns a clear unsupported-backend
error before network I/O.

## Embedded resource validation

`BrowserPage::validate_external_resources()` checks that every external script,
stylesheet, image, `modulepreload`, and preload reference in the page has a
registered deterministic resource. This lets agents fail fast on missing
production bundles before executing app code or taking screenshots.

When `BrowserPage::run_scripts()` is called, registered `<script src="...">`
resources, including `type="module"` entries, are inlined at their original
script element and executed by the in-tree JavaScript runtime in document order.
Static module imports are resolved from the registered resource set and executed
before the importing module. Relative imports can match registered page paths or
fully resolved URLs. Default imports, named import aliases, `export default`,
and bundle-style `export { local as Name }` lists are rewritten into runtime
bindings for the existing JavaScript engine. Literal dynamic imports such as
`import("./chunk.js")` are resolved through the same registry and rewritten to a
fulfilled namespace promise. Common arrow functions in module resources are
rewritten to function expressions before evaluation. Passive preload links are
validated but do not execute by themselves.

## Page snapshot schema target

The native host should make `page_snapshot()` return a compact map with:

- url, title, viewport, scroll_position, focused_element, selected_element, visible_text
- dom_tree, accessibility_tree
- elements[] containing selector_candidates, tag, id, classes, role, accessible_name, text, attributes, bounding_box, visible, enabled, checked, selected, computed_styles, event_listeners_if_available
- forms, links, buttons, inputs, images, scripts, stylesheets, framework_roots

This file documents the host-facing API; the in-tree browser modules remain the
runtime that production browser validation must exercise and improve.
