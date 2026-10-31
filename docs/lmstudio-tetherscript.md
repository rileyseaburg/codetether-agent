# LM Studio validation with TetherScript

These are executable model checks, not mock HTTP routes. Run them from the
repository root with the local SSH tunnel active and Gemma loaded on the Mac.
They do not require rebuilding CodeTether after editing the scripts.

## Checks

- `examples/tetherscript/lmstudio_gemma.tether`: `budget_at`, `budget_from`,
  and `chat_at` check loaded capacity, reject missing capacity and call chat.
- `examples/tetherscript/lmstudio_validate.tether`: `tool_roundtrip_at` asks
  the real model for a tool call, validates the tool name/path, reads only
  `docs/lmstudio.md`, sends its real first five lines as `role=tool`, and
  checks the returned title. It is a narrow scripted tool executor, not the
  whole native TUI tool registry.
- `stream_at` verifies SSE content, usage, reasoning visibility and `[DONE]`.
  Usage is checked against the 256-token output cap and actual loaded context.
- `tool_stream_at` reconstructs tool-call IDs and arguments across streamed
  fragments; evidence is `artifacts/lmstudio-local/tether-tool-stream.json`.

Run each live hook through the installed CodeTether tool runtime:

```sh
RUST_LOG=error codetether mcp call \
  --command 'codetether mcp serve' --tool tetherscript_plugin \
  --arguments '{"path":"examples/tetherscript/lmstudio_validate.tether","hook":"tool_roundtrip_at","args":["http://127.0.0.1:18080","google/gemma-4-12b-qat"],"timeout_secs":60}'
```

Replace the hook with `stream_at` for the streaming check. Evidence is written
to `artifacts/lmstudio-local/tether-tool-roundtrip.json` and `tether-stream.json`.
The scripts use `process_run` with curl for longer calls: the builtin HTTP
client has a fixed 15-second socket timeout and calls that timeout “empty request”.

These checks establish the real model's protocol behavior. They do not by
themselves establish that a new native TUI adapter build is installed or working.