# Codex task-quality parity benchmark

This differential benchmark runs OpenAI Codex CLI and CodeTether against fresh
copies of the same small Rust repositories. It measures whether each agent exits
successfully, passes protected acceptance tests and Clippy, how long it takes,
and which files it changes.

Both sides must use equivalent models. Model names differ between clients, so
they are explicit rather than inherited from user configuration:

```bash
python3 scripts/codex_parity_benchmark.py \
  --codex-model gpt-5.3-codex \
  --codetether-model openai-codex/gpt-5.3-codex \
  --dry-run
```

Remove `--dry-run` only when credentials and model-call cost are intentional.
Real runs default to `artifacts/codex-parity/<UTC timestamp>/`. The harness
refuses to overwrite that directory and retains both workspaces, raw JSONL and
stderr, binary git patches, deterministic check output, versions, and the final
`report.json`.

The primary parity metric is per-case acceptance pass rate. Duration is
secondary because provider load varies. Review retained patches and transcripts
when both agents pass: equal tests do not imply equal implementation quality.
The report records the CodeTether-minus-Codex pass-rate gap; parity is zero or
better on the same case set.

Cases are declared in `benchmarks/codex-parity/cases.toml`. Each fixture keeps
acceptance tests outside `src/`, and the harness verifies protected files remain
byte-for-byte unchanged before crediting a pass.
