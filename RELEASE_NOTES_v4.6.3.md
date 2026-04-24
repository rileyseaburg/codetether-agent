## What's New

- **TetherScript Plugin Runtime** (#81) — Load and execute community-authored tool plugins from JSON manifests. Includes schema conversion, path-escape protection, resource limits, and a sandboxed runner. See `docs/tetherscript_dependency.md`.

- **Browser Control CLI** (#92) — New `browserctl` sub-command exposes all browser automation actions (navigate, click, screenshot, network capture, replay) from the terminal without starting the TUI.

- **Voice Input Tool** — Real-time microphone capture to transcription pipeline (`voice_input` tool). Adds recorder, encoder, streaming input, and TUI keybinding integration.

- **Voice Streaming Output** — Streaming text-to-speech playback via the `voice_stream` tool with a dedicated audio player and speak-stream action.

- **Image Clipboard Bridge** (#78) — Paste images from the SSH clipboard into the TUI. New `image_clipboard` module handles capture, base64 encoding, and data-URL construction.

- **Ralph Verification Steps** (#87) — Stories can now declare structured verification steps (shell commands, file assertions, URL health checks). Ralph gates story completion on all steps passing.

- **Swarm Tool Policy** (#90) — Declarative read-only tool policy for swarm sub-agents. Definitions, registry, and injected prompt sections enforce that spawned agents cannot mutate the workspace.

- **Provider Redundancy Policy** — New `provider_fallback_policy` module governs provider selection when the primary provider is unavailable.

## Bug Fixes

- **Fail-closed policy authorization** (#88) — OPA policy checks now deny by default when the policy engine is unreachable, preventing silent allow-through.
- **Harden plugin sandbox verification** (#91) — Stricter Ed25519 signature and SHA-256 integrity checks for TetherScript manifests, plus separated network, path, and limits modules.
- **Prevent interactive bash auth prompts** (#80) — Bash tool forces `GIT_TERMINAL_PROMPT=0` and strips askpass helpers so credential challenges never block the agent loop.
- **Explain npx TLS certificate failures** (#82) — New `tls_remediation.js` helper detects self-signed and enterprise MITM certs and prints actionable guidance.
- **Align OpenAI Codex ChatGPT model defaults** (#68) — Fixes incorrect model ID mapping that sent requests to the wrong endpoint.
- **Remove disconnected worktree split** (#85) — Eliminates stale worktree branch and merge logic that left orphaned refs after errors.
- **Mark dynamic tools discovery-only** (#84) — Dynamic MCP tools no longer appear as executable in tool listings until fully resolved.
- **Gate Ralph stories on verification steps** (#87) — Stories with verification blocks no longer auto-pass without running them.

## Changes

- **Worktree module removed** — cleanup, helpers, integrity, manager, merge, and types sub-modules deleted; worktree lifecycle is now handled inline by consumers.
- **Global file-line ratchet enforced** (#86) — `check_file_limits.sh` updated; CI now rejects PRs that grow files beyond the 50-line soft limit.
- **Non-interactive Bash variant** — New `bash_noninteractive.rs` provides a prompt-free execution path used by swarm and Ralph.
- **Policy user extraction** — `PolicyUser` moved to its own module with tests for role-based and tenant-isolated access.
- **Tool contract endpoint** — New `tool_contract.rs` in the server layer exposes tool metadata over HTTP.
- **K8s executor tool policy passthrough** — Kubernetes-spawned sub-agents inherit the swarm tool policy.
- **147 files changed**, +4,351 minus 1,141 lines across CLI, TUI, server, tools, swarm, Ralph, and provider layers.
