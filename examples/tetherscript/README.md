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
