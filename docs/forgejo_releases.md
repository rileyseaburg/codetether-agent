# Forgejo releases

Release source: `https://forgejo.quantum-forge.io/riley/codetether-agent`.
The `.forgejo/workflows/release.yml` entry point runs entirely in CI. Do not
run local Cargo builds to substitute for this workflow.
Use this workflow instead of local `release.sh`, which compiles and installs.

## Trigger

Push a trusted candidate branch named `release/forgejo-*`, or dispatch
`release.yml` on the desired ref through Forgejo Actions. The version comes
from `Cargo.toml`; the workflow does not modify it or publish to crates.io.

For authenticated API access use `forgejo-cli`. The configured CI credential
is Vault path `secret/forgejo/spotlessbinco-ci-secrets`, field `GITOPS_TOKEN`.
The generic `kv/forgejo/*` bot tokens do not grant write access to this repo.
Never print the credential or commit it:

```bash
set +x
FORGEJO_API_BASE=https://forgejo.quantum-forge.io/api/v1
FORGEJO_API_KEY="$(vault kv get -field=GITOPS_TOKEN secret/forgejo/spotlessbinco-ci-secrets)"
forgejo-cli --base "$FORGEJO_API_BASE" --token "$FORGEJO_API_KEY" post \
  /repos/riley/codetether-agent/actions/workflows/release.yml/dispatches \
  '{"ref":"release/forgejo-v4.7.5-dev.49"}'
```

## Runners and gates

- Linux verification/build/publication: `spotlessbinco-k8s-dind-privileged`.
- Windows GNU (`x86_64-pc-windows-gnu`): cross-built with Docker Buildx on
  `spotlessbinco-k8s-dind-privileged`, using `docker/release/windows.Dockerfile`.
  The CI job requires the Dockerfile build to use `--locked` and publishes
  both versioned `.exe` and `.zip` files. No native Windows runner or Windows
  runtime smoke test is used by this cross-build.
- Apple Silicon and Intel macOS: `macOS`.
- Runner registration/visibility must include this repository. macOS runner
  availability must be checked before claiming a build has run.
- Rust 1.95 is selected explicitly. Verification includes formatting, release
  shell regressions, clippy, and the full serial library/integration test gate.
  Platform builds run alongside verification; publication requires both.
- Forgejo-compatible v3 artifact actions transfer the outputs. Publication
  requires all four platform archives plus the GNU Windows executable and
  creates a SHA-256 manifest.

The Forgejo release is a prerelease at `v<Cargo.toml version>`. macOS artifacts
are **unsigned**; this workflow does not claim Apple signing/notarization.
CI logs, artifact IDs, and the resulting release URL are the execution proof.