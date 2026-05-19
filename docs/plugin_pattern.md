# CodeTether Plugin Pattern

CodeTether agents call the `tetherscript_plugin` tool to execute `.tether`
script hooks at runtime. This document describes the contract between the
agent (caller), the tool (dispatch), and the plugin (`.tether` file).

## Architecture

```
Agent → ToolCall JSON → tetherscript_plugin tool
                            ↓
                      Interpreter::new()  (full builtins)
                            ↓
                      lexer → parser → run_repl
                            ↓
                      interp.call(hook_name, args)
                            ↓
                      TetherScript Interpreter (blocking thread)
                            ↓
                      ToolResult → agent context
```

The tool uses `Interpreter::new()` which includes the **full** built-in set
(JSON, JS eval, browser rendering, HTTP, filesystem, etc.). The TetherScript
crate's `PluginHost` uses `Interpreter::new_sandboxed()` which omits browser
and JS builtins, so CodeTether constructs its own interpreter to expose
all features to plugins.

## Tool Call Contract

The agent sends a JSON tool call with these fields:

| Field        | Type     | Required | Description |
|--------------|----------|----------|-------------|
| `path`       | `string` | one of   | Path to `.tether` file (relative to workspace root) |
| `source`     | `string` | one of   | Inline TetherScript source (used instead of `path`) |
| `hook`       | `string` | **yes**  | Top-level function name to call |
| `args`       | `array`  | no       | JSON arguments converted to TetherScript values |
| `timeout_secs` | `int`  | no       | Wall-clock timeout (capped at 60s, default 10s) |
| `grant_browser` | `string` | no    | Browser bridge endpoint to grant live browser capability |
| `browser_origin` | `array` | no    | Allowed origins for browser capability |
| `browser_scope` | `array` | no     | Allowed scopes for browser capability |

### File-based invocation (preferred)

```json
{
  "path": "examples/tetherscript/browser_render.tether",
  "hook": "render",
  "args": ["<h1>Hello</h1>"]
}
```

### Inline invocation (one-shot scripts)

```json
{
  "source": "fn validate(name) { return Ok(\"hello \" + name) }",
  "hook": "validate",
  "args": ["codetether"]
}
```

## Writing a Plugin File

A plugin file is a `.tether` source file that defines top-level functions.
Each exported function becomes a callable **hook**.

### Minimal plugin

```
// my_plugin.tether

fn greet(name) {
    return Ok("hello " + name)
}
```

Called as:

```json
{
  "path": "my_plugin.tether",
  "hook": "greet",
  "args": ["world"]
}
```

### Returning structured data

Use TetherScript `map()` to build structured results, then `json_encode`
to return JSON the agent can parse:

```
fn analyze(text) {
    let m = map()
    m["length"] = len(text)
    m["has_secret"] = text.contains("BEGIN PRIVATE KEY")
    return json_encode(m)
}
```

### Using built-in functions

TetherScript ships with built-in functions that plugins can call directly
(no imports needed). The full list is in the TetherScript README. Key ones:

- **JSON**: `json_parse`, `json_encode`, `json_encode_pretty`
- **JS eval** (alpha.8): `js_eval(source)`
- **Browser** (alpha.8): `browser_render`, `browser_eval_js`, `browser_run_scripts`,
  `browser_snapshot`, `browser_query_selector`, `browser_text_content`,
  `browser_compatibility_report`
- **HTTP**: `http_get`, `http_post`, `http_request`
- **FS**: `fs_read`, `fs_write`, `fs_exists`, `fs_list`
- **Strings**: `.contains()`, `.split()`, `.replace()`, `.upper()`, `.lower()`

### Return values

Hooks should return one of:

- `Ok(value)` — success, agent sees `{"ok": value}` in metadata
- `Err(message)` — failure, agent sees `{"err": message}`
- A raw value (string, map, etc.) — returned as-is
- `json_encode(map)` — structured JSON string in output

## Result Handling

The tool returns a `ToolResult` with:

| Field     | Description |
|-----------|-------------|
| `output`  | Concatenation of `println()` output and the return value display |
| `success` | `true` unless the hook returned `Err(...)` |
| `metadata.value` | The raw TetherScript return value converted to JSON |

### Success response example

```
output: "Hello from JS!\n{\"ok\":true,\"result\":\"Hello from JS!\"}"
success: true
metadata: { "value": { "ok": true, "result": "Hello from JS!" } }
```

## Testing Plugins Through the Runtime

Plugins are tested through the `TetherScriptPluginTool` trait object,
not by running `tetherscript run` directly. This exercises the full
CodeTether dispatch path (load → parse → call → convert → result).

### Inline source test

```rust
#[tokio::test]
async fn test_inline_hook() {
    let tool = TetherScriptPluginTool::new();
    let result = tool.execute(json!({
        "source": "fn add(a, b) { return Ok(a + b) }",
        "hook": "add",
        "args": [3, 4]
    })).await.unwrap();
    assert!(result.success);
    assert_eq!(result.metadata["value"]["ok"], 7);
}
```

### File-based test

```rust
#[tokio::test]
async fn test_file_hook() {
    let tool = TetherScriptPluginTool::new();
    let result = tool.execute(json!({
        "path": "examples/tetherscript/guardrails.tether",
        "hook": "scan_text",
        "args": ["BEGIN PRIVATE KEY"]
    })).await.unwrap();
    assert!(result.success);
}
```

### Tempdir test (isolated)

```rust
#[tokio::test]
async fn test_isolated_plugin() {
    let dir = tempfile::tempdir().unwrap();
    tokio::fs::write(dir.path().join("test.tether"),
        "fn check(v) { return Ok(v > 0) }"
    ).await.unwrap();
    let tool = TetherScriptPluginTool::with_root(dir.path().to_path_buf());
    let result = tool.execute(json!({
        "path": "test.tether",
        "hook": "check",
        "args": [42]
    })).await.unwrap();
    assert!(result.success);
}
```

## Adding a New Plugin

1. Create `examples/tetherscript/your_plugin.tether` with named hook functions
2. Add usage examples to `examples/tetherscript/README.md`
3. Add a test in `src/tool/tetherscript/tests/` if the plugin uses
   runtime features (browser, JS, etc.)
4. The agent can immediately call it — no Rust changes or rebuild needed

## Available Plugin Files

| File | Hooks | Purpose |
|------|-------|---------|
| `guardrails.tether` | `allow_path`, `scan_text` | Security policy checks |
| `task_score.tether` | `score`, `classify` | Task prioritization |
| `test_output.tether` | `cargo_status`, `next_action` | Test result routing |
| `pr_summary.tether` | `title`, `checklist` | PR description helpers |
| `release_note.tether` | `summarize` | Release note generation |
| `deepseek_repair.tether` | `repair_msg` | Fix null reasoning content |
| `cerebras_chat.tether` | `complete`, `models` | Cerebras LLM provider |
| `lmstudio_gemma.tether` | `chat`, `chat_at`, `complete`, `models`, `models_at` | LM Studio Gemma helper |
| `js_eval.tether` | `eval`, `eval_json` | JavaScript evaluation |
| `browser_render.tether` | `render`, `render_with_css`, `layout`, `display_list`, `snapshot`, `snapshot_wide` | HTML/CSS rendering |
| `browser_dom.tether` | `text`, `query`, `styles`, `extract_links`, `extract_text` | DOM querying |
| `browser_js.tether` | `eval_js`, `run_scripts`, `compat`, `interactive` | Browser JS runtime |
| `browser_agentic_debug.tether` | `verify_checkout`, `main` | Live browser agentic debugging (alpha.10) |
