# Forgejo releases

Release source: `https://forgejo.quantum-forge.io/riley/codetether-agent`.
The `.forgejo/workflows/release.yml` entry point runs entirely in CI. Do not
run local Cargo builds to substitute for this workflow.

## Trigger

Push a trusted candidate branch named `release/forgejo-*`, or dispatch
`release.yml` on the desired ref through Forgejo Actions. The version comes
from `Cargo.toml`; the workflow does not modify it or publish to crates.io.

For authenticated API access use `forgejo-cli` with an already provisioned
repository-scoped credential. Never print the credential or commit it:

```bash
forgejo-cli --base "$FORGEJO_API_BASE" --token "$FORGEJO_API_KEY" post \
  /repos/riley/codetether-agent/actions/workflows/release.yml/dispatches \
  '{"ref":"release/forgejo-v4.7.5-dev.49"}'
```

## Runners and gates

- Linux verification/build/publication: `spotlessbinco-k8s`.
- Windows MSVC: runner labels `self-hosted` and `Windows`.
- Apple Silicon and Intel macOS: `macOS`.
- Runner registration/visibility must include this repository. The Windows
  and macOS labels match the existing platform workflow conventions; their
  current availability must be checked before claiming a build has run.
- Rust 1.95 is selected explicitly. Verification includes formatting, release
  shell regressions, clippy, and the full serial library/integration test gate.
- Forgejo-compatible v3 artifact actions transfer the outputs. Publication
  requires all four platform archives and creates a SHA-256 manifest.

The Forgejo release is a prerelease at `v<Cargo.toml version>`. macOS artifacts
are **unsigned**; this workflow does not claim Apple signing/notarization.
CI logs, artifact IDs, and the resulting release URL are the execution proof.
