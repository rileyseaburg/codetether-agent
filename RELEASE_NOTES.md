# v4.7.4

## What's New

- **Session picker is far easier to navigate.** Rows now show the project
  directory and a relative age (`3m ago`, `2h ago`, `5d ago`) alongside the
  8-char id and title, so sessions that share a near-identical first-message
  title are visually distinct instead of indistinguishable.

## Bug Fixes

- **Bedrock: `claude-sonnet-4-6` now resolves to a valid inference profile.**
  The alias table mapped it to `us.anthropic.claude-sonnet-4-6-v1:0`, which
  Bedrock rejects with `400 ValidationException: The provided model
  identifier is invalid`. The canonical id has no `-v1:0` suffix. Verified
  live against Bedrock; opus-4-5/4-6/4-7/4-8, sonnet-4-5/5 and haiku-4-5 were
  already correct and are unchanged.
- **Provider failover no longer jumps straight to `zai`.** A failing Bedrock
  model now retries sibling Bedrock models (Sonnet/Opus/Haiku) before falling
  through to the cross-provider tail.
- **Windows clipboard image paste over SSH restored.** Reads `CF_DIB`, wraps
  it in a BMP header, and re-encodes as a PNG data URL so `codetether
  clipboard image` works again when SSH'd from Windows. Ctrl+V probes the
  real clipboard even under SSH (X11/Wayland forwarding) and embedded image
  data URLs are extracted even when chunked or mixed with caption text.
- **Transparent restart on premature stream termination** so a dropped
  upstream connection no longer aborts the turn.
- **Provider account-exhaustion detection** skips same-provider retries when a
  plan/billing failure is detected, leaving the dead provider behind.
- **stdout log pollution fixed**: tracing logs go to stderr so MCP stdio
  JSON-RPC and piped `codetether run` output are no longer corrupted.

## Changes

- Faster session picker loads via `(mtime,size)`-cached listings.
- Test isolation: reset leaked global access-mode/grant state between policy
  tests to eliminate spurious parallel-run failures.
- Release CI: workspace-aware publishing, `build-essential` / `libasound2-dev`
  / `protoc` for Linux release binaries, and an attach-to-existing tag mode.
- New reusable TetherScript probe (`examples/tetherscript/bedrock_model_probe.tether`)
  that validates Bedrock model ids live via `aws bedrock-runtime converse`.
