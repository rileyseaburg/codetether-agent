# Gemini Web native tool inference

The chat harness describes an external tool executor, shows a valid JSON
`<tool_call>` example, and asks for one call followed by a stop. A subsequent
`Role::Tool` message supplies the real result before the next call or final
answer. Advertised-tool allowlisting, JSON-schema validation, and session-ID
guards still apply; these prompt changes do not expand tool permissions.

## Findings and transport

In a six-trial curl experiment with two synthetic tools, schema-only prompting
produced no completed tool round trips (0/2). Explicit call/stop/wait instructions
and an example-driven variant each produced 2/2. This is a small prompt experiment,
not an assurance of universal tool reliability or backend model identity.

The native Rust test then reproduced the reported HTTP/2 `PROTOCOL_ERROR` during
the home-page bootstrap, before inference. The Gemini chat client now selects
HTTP/1.1 up front, matching its image client. This is specific to Gemini Web;
other providers are unchanged and no automatic generation-POST replay is added.

## Repeatable tests

```sh
cargo test -p codetether-gemini-web-protocol -p codetether-provider-gemini-web --lib
```

Those tests cover the executable JSON example, advertised calls, streamed call
start/delta/end IDs, and actual tool-result history. The live test is ignored by
default: it contacts Google and reads the configured Gemini cookies using the
Vault CLI. It never prints or saves cookies or session tokens.

```sh
CODETETHER_GEMINI_PROOF_DIR="$PWD/artifacts/gemini-native-$(date -u +%Y%m%dT%H%M%SZ)" \
  cargo test -p codetether-provider-gemini-web --lib live_native_fast_tool_roundtrip \
  -- --ignored --nocapture
```

The live test uses the production Rust provider, prompt renderer, reqwest client,
tool parser, and stream events. It executes only local fixture tools:
`lookup_probe` returns withheld data, `multiply` must consume that quantity, and
the final answer must include the withheld code and computed product. Evidence
is saved as `native-fast-tools.json`, including sanitized failures. It tests
Fast's provider-level streaming loop, not the full TUI, arbitrary tools, or other
models. Source changes require an updated binary; a running TUI is not upgraded.

## Recorded evidence (2026-11-09)

Evidence level: native Rust HTTP requests to live Gemini, with mocked local tools.
The final revision produced two successful lookup/multiply/final-answer round trips.

- [Original HTTP/2 bootstrap failure](../artifacts/gemini-native-baseline-20261109T162606Z/native-fast-tools.json)
- [Initial HTTP/1.1 round trip](../artifacts/gemini-native-http1-20261109T162653Z/native-fast-tools.json)
- [Intermediate missing-call failure](../artifacts/gemini-native-repeat-20261109T162955Z/native-fast-tools.json)
- [Round trip with partial-turn tracing enabled](../artifacts/gemini-native-trace-20261109T163151Z/native-fast-tools.json)
- [Trace showing skipped multiply call](../artifacts/gemini-native-stability-20261109T163235Z-1/native-fast-tools.json)
- [Second intermediate stability run](../artifacts/gemini-native-stability-20261109T163235Z-2/native-fast-tools.json)
- [Final required-tool prompt, run 1](../artifacts/gemini-native-required-tools-20261109T163454Z-1/native-fast-tools.json)
- [Final required-tool prompt, run 2](../artifacts/gemini-native-required-tools-20261109T163454Z-2/native-fast-tools.json)

The missing-call trace showed the model calculating instead of calling multiply,
so the final prompt explicitly forbids skipping user-required tools via mental arithmetic.
Two successful runs are bounded evidence, not a guarantee of universal model compliance.