# TetherScript Plugin Pack

These scripts run through the existing `tetherscript_plugin` tool. They are
small examples of useful project-local automation without adding Rust code.

## Guardrails

Deny sensitive paths:

```json
{
  "path": "examples/tetherscript/guardrails.tether",
  "hook": "allow_path",
  "args": [".env.local"]
}
```

Scan text for obvious secrets:

```json
{
  "path": "examples/tetherscript/guardrails.tether",
  "hook": "scan_text",
  "args": ["BEGIN PRIVATE KEY"]
}
```

## Task scoring

```json
{
  "path": "examples/tetherscript/task_score.tether",
  "hook": "score",
  "args": ["security bug in auth", 10]
}
```

```json
{
  "path": "examples/tetherscript/task_score.tether",
  "hook": "classify",
  "args": ["bug in parser"]
}
```

## Test output routing

```json
{
  "path": "examples/tetherscript/test_output.tether",
  "hook": "cargo_status",
  "args": ["test result: ok. 12 passed"]
}
```

```json
{
  "path": "examples/tetherscript/test_output.tether",
  "hook": "next_action",
  "args": ["failed"]
}
```

## PR summary helpers

```json
{
  "path": "examples/tetherscript/pr_summary.tether",
  "hook": "title",
  "args": ["feat", "tetherscript", "add reusable plugin examples"]
}
```

```json
{
  "path": "examples/tetherscript/pr_summary.tether",
  "hook": "checklist",
  "args": ["manual smoke tests", "updated examples README"]
}
```

## Release note

```json
{
  "path": "examples/tetherscript/release_note.tether",
  "hook": "summarize",
  "args": ["1.2.3", "Added TetherScript examples"]
}
```

## JavaScript evaluation (alpha.8)

Run JavaScript through the built-in zero-dependency interpreter:

```json
{
  "path": "examples/tetherscript/js_eval.tether",
  "hook": "eval",
  "args": ["1 + 2 * 3"]
}
```

```json
{
  "path": "examples/tetherscript/js_eval.tether",
  "hook": "eval_json",
  "args": ["({ name: 'test', count: 42 })"]
}
```

## Headless browser

Render HTML to text, take snapshots, and compute layout — all without a real browser:

```json
{
  "path": "examples/tetherscript/browser_render.tether",
  "hook": "render",
  "args": ["<h1>Hello</h1><p>World</p>"]
}
```

```json
{
  "path": "examples/tetherscript/browser_render.tether",
  "hook": "render_with_css",
  "args": ["<h1>Hello</h1>", "h1 { color: red }"]
}
```

```json
{
  "path": "examples/tetherscript/browser_render.tether",
  "hook": "snapshot",
  "args": ["<div><p>text</p></div>"]
}
```

## DOM querying

Extract text and elements from HTML using CSS selectors:

```json
{
  "path": "examples/tetherscript/browser_dom.tether",
  "hook": "extract_text",
  "args": ["<h1>Title</h1><p>Body text</p>"]
}
```

```json
{
  "path": "examples/tetherscript/browser_dom.tether",
  "hook": "query",
  "args": ["<div class='x'>hi</div>", ".x"]
}
```

## Browser JavaScript runtime

Execute JS with a full DOM (document, window, localStorage, timers):

```json
{
  "path": "examples/tetherscript/browser_js.tether",
  "hook": "eval_js",
  "args": ["<div id='app'></div>", "document.getElementById('app').textContent = 'hello'"]
}
```

```json
{
  "path": "examples/tetherscript/browser_js.tether",
  "hook": "run_scripts",
  "args": ["<script>console.log('hi')</script><p>done</p>"]
}
```

```json
{
  "path": "examples/tetherscript/browser_js.tether",
  "hook": "compat",
  "args": []
}
```

### Interactive page with JS + rendered output

```json
{
  "path": "examples/tetherscript/browser_js.tether",
  "hook": "interactive",
  "args": ["<div id='out'></div>", "document.getElementById('out').textContent = Date.now()"]
}
```
