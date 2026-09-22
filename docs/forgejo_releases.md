# Forgejo releases

Release source: `https://forgejo.quantum-forge.io/riley/codetether-agent`; [GitHub distributes mirrored code and release assets](forgejo_mirror.md).
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
Never print, commit, or pass the credential in command arguments:

```bash
(
set +x
FORGEJO_API_BASE=https://forgejo.quantum-forge.io/api/v1
FORGEJO_TOKEN="$(vault kv get -field=GITOPS_TOKEN secret/forgejo/spotlessbinco-ci-secrets)" || exit
export FORGEJO_TOKEN
trap 'unset FORGEJO_TOKEN' EXIT
forgejo-cli --base "$FORGEJO_API_BASE" post \
  /repos/riley/codetether-agent/actions/workflows/release.yml/dispatches \
  '{"ref":"release/forgejo-v4.7.5-dev.49"}'
)
```

## Runners and gates

- Linux verification/build/publication: `codetether-release-proxmox`.
  This exclusive label belongs to global runner 174,
  `proxmox-node-ephemeral-lxc`, backed by Proxmox LXC 300 (28 GiB RAM,
  10 cores, capacity one). Jobs use `lxc://debian:bookworm:lxc docker`.
  Do not substitute its generic labels: those also match Kubernetes runners.
  The label is configured in `/etc/forgejo-runner/config.yml` inside LXC 300;
  runner configuration changes require an idle-service restart. Existing labels
  remain available to other projects. Capacity one serializes Linux-side jobs.
- Windows GNU (`x86_64-pc-windows-gnu`): cross-built with Docker Buildx on
  `codetether-release-proxmox`, using `docker/release/windows.Dockerfile` and
  the ephemeral job's local Unix Docker socket, not the Kubernetes TCP daemon.
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
- Publication pins `forgejo-release` v2.1.0 by commit SHA. The floating `v2`
  branch requires newer `forge.*` context and Node 24 support that runner
  v0.2.11 does not provide; do not float this pin without upgrading the runner.

The Forgejo release is a prerelease at `v<Cargo.toml version>`. macOS artifacts
are **unsigned**; this workflow does not claim Apple signing/notarization.
CI logs, artifact IDs, and the resulting release URL are the execution proof.