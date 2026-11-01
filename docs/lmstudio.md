# Local LM Studio on the Mac

LM Studio is accessed as provider `lmstudio` through the OpenAI-compatible
adapter. The Mac currently serves `google/gemma-4-12b-qat` on loopback port
8080, with a loaded context of 32768 tokens. These are instance settings,
not the model architecture's advertised 262144-token maximum.

## Local connection

`scripts/dev/codetether-lmstudio-tunnel.service` is a user systemd unit that
forwards local `127.0.0.1:18080` to `ssh mac`'s `127.0.0.1:8080`. It uses the
existing SSH configuration and restarts on connection failure. The API is
not exposed to the LAN.

```sh
install -D -m 644 scripts/dev/codetether-lmstudio-tunnel.service \
  ~/.config/systemd/user/codetether-lmstudio-tunnel.service
systemctl --user daemon-reload
systemctl --user enable --now codetether-lmstudio-tunnel.service
codetether models --provider lmstudio --json
```

The existing Vault entry `secret/codetether/providers/lmstudio` has its
`base_url` set to `http://127.0.0.1:18080/v1`; its other fields are preserved.
That loopback URL is specific to clients running on this machine. Other
machines or container network namespaces need their own reachable endpoint.

## TUI

After installing the updated binary, use `bash scripts/dev/lmstudio-tui.sh`
from this repository, or install it as a local launcher:

```sh
install -m 755 scripts/dev/lmstudio-tui.sh ~/.local/bin/codetether-lmstudio
codetether-lmstudio
```

The launcher does not change normal CodeTether defaults. It selects Gemma,
defaults output to 4096 tokens and tool output to 4096 bytes, and uses the automatic
small LM Studio schema profile. Other tools remain available via `list_tools`.

## Context safety

Discovery and requests read fresh `/api/v0/models` loaded-instance metadata.
Unloaded chat models and embedding models are excluded from the chat picker.
History selection reserves half the loaded window, counting system instructions,
schemas and pinned constraints before history. Required text is not silently
discarded to make a request fit. The final text-only guard estimates tokens,
reserves a safety margin, caps output and rejects oversized requests locally.
Estimation is not an exact Gemma tokenizer; media inputs are rejected.
See [TetherScript validation](lmstudio-tetherscript.md) for repeatable live checks.
See [runtime limits and queue](lmstudio-runtime.md) for configuration and deployment boundaries,
and [resumable text jobs](lmstudio-jobs.md) for bounded multi-step generation.