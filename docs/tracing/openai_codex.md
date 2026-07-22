# OpenAI Codex trace capture

Authenticate once, then run the trace wrapper:

```bash
codetether auth codex --device-code
scripts/trace_openai_codex.sh 'Explain this repository'
```

Artifacts are retained under `artifacts/openai-codex-trace/<UTC timestamp>/`:

- `metadata.txt` records model, versions, and the active tracing filter.
- `prompt.txt` records the complete prompt.
- `stdout.log` records the model response.
- `stderr.log` records CodeTether, provider, HTTP, and WebSocket diagnostics.
- `network.strace.*` records timestamped socket calls for every process/thread.
- `network.strace.*.jsonl` preserves every source byte and adds lexical tokens.
- `network.strace.*.jsonl.manifest.json` records source size, lines, and SHA-256.
- `result.txt` records the final exit status.

Override the destination with `CODETETHER_TRACE_DIR=/secure/path`. The wrapper
creates the destination with mode `0700` and never deletes trace artifacts.

## Security boundary

OAuth credentials remain in CodeTether's configured secret backend. The
wrapper does not print environment values, Authorization headers, tokens, or
the ChatGPT account ID. Prompt and response content are stored unredacted.

`strace` captures socket destinations, timing, flags, sizes, and encrypted TLS
records available through network syscalls. It cannot decode HTTPS payloads.
Capturing decrypted HTTP would expose bearer credentials and content and is
intentionally outside this workflow.

Socket strings use unlimited length and hexadecimal escaping, preventing
capture-time abbreviation while preserving arbitrary encrypted bytes.

Each JSONL record contains exact Base64 source bytes, decoded raw text, its line
terminator, all whitespace and non-whitespace tokens with offsets, and parsed
syscall fields when recognized. Unrecognized records remain losslessly encoded.

## Optional packet capture

When packet-level evidence is required and you are authorized to capture it,
start this in a second terminal before the wrapper:

```bash
sudo tcpdump -i any -s 0 -w openai-codex.pcap 'tcp port 443'
```

Stop it with `Ctrl+C` after the run. The PCAP still contains TLS-encrypted
payloads and may include unrelated HTTPS traffic, so store it securely.

## Human trace dashboard

Launch the latest parsed trace in the TetherScript-hosted Trace Lens UI:

```bash
scripts/open_trace_ui.sh
```

Pass a specific `network.strace.*.jsonl` path to view an older capture. The
launcher opens port `8789` using the SSH host address when required, prefers
Tera for its shell, and losslessly
chunks large traces beneath their artifact directory for TetherScript's 1 MiB
read limit. TetherScript listens on all interfaces, so a random per-run token
and `HttpOnly` cookie reject requests that did not originate from the launch
URL. Use `TRACE_UI_NO_OPEN=1` when running a headless smoke test.
Older standalone binaries without Tera use the same template with a safe
placeholder fallback; this project enables Tera in its dependency features.
