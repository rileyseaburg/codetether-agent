# Forgejo author protocol completion audit

Date: 2026-07-19

## Scope

The audited protocol is `codetether.forgejo-author.v1` across:

- `/home/riley/spotlessbinco` (trusted Forgejo workflow),
- `/home/riley/A2A-Server-MCP` (durable protocol authority), and
- `/home/riley/A2A-Server-MCP/codetether-agent` (canonical worker).

## Requirement evidence

| Requirement | Current-state evidence | Result |
|---|---|---|
| Repository/agent neutrality | Repository, host, reviewer, model, API URL, candidate principals, and worker slot are configuration. A focused source scan found no Spotless/Quantum Forge/Riley identity in protocol modules. | static/local: satisfied |
| Fail-closed admission | Server re-fetches the open PR and exact head from an allowlisted Forgejo API, verifies Forgejo signer plus CodeTether HMAC, strips client trust fields, requires durable storage and a live canonical worker, and rejects protocol downgrade. | mocked local: covered by the 89-test protocol suite |
| Cryptographic worker binding | Registration, liveness, claim, release, status, output, resume, and progress proofs are key/tenant/identity/action/resource bound. Mutation resources include the exact JSON-body SHA-256. | focused CI-like: Rust proof/vector tests; mocked local: Python endpoint tests |
| Principal isolation | The provenance key is bound to a tenant and labeled bearer. Reads, listings, output streams, cancellation, claims, and mutations enforce the bound principal and claim owner. | mocked local: task-access, tenant, claim, and mutation tests |
| Server idempotency | Task identity is derived from trusted tenant/Forgejo/repository/PR/head/stage/agent fields; PostgreSQL conflict behavior preserves the first task. Canonical claims use a conditional PostgreSQL reservation and recognize same-worker retries. | mocked local: concurrency, dedupe, upsert, and claim-reservation tests |
| Universal reviewer-to-author flow | The workflow selects an independent configured reviewer, forces targeted reviews through `/v1/agent/tasks`, validates exact diff coverage and structured output, rechecks head freshness, and routes only a verified canonical author principal. | mocked local: nine shell suites and six Playwright source-contract cases |
| Operational configuration | Argo declares the server provenance-key secret and Forgejo host map. Workflow API URL/token and all repository-specific deployment values are explicit variables/secrets. | static/local: YAML parsing and source inspection |

## Final verification

- **focused CI-like** — `cargo fmt --all -- --check`: exit 0.
- **focused CI-like** — `./check_file_limits.sh`: exit 0.
- **focused CI-like** — `cargo test --lib worker_security`: 2 passed, 0 failed.
- **mocked local** — focused Python protocol/worker command: 89 passed, 0 failed;
  11 pre-existing deprecation warnings remain.
- **mocked local** — nine `codetether-review-*.test.sh` suites: exit 0.
- **mocked local** — `npx playwright test
  tests/playwright/codetether/pr-review-workflow.spec.ts`: 6 passed.
- **static/local** — `git diff --check` in all three repositories: exit 0.
- **static/local** — Argo and Forgejo workflow YAML parsing: exit 0.
- **static/local** — protocol identity scan: no repository-specific identities.

## Evidence boundary

No live deployment, Argo synchronization, real Forgejo PR execution, or real
platform upload was requested or run. The completion claim is source-level and
focused CI-like/mocked-local, not live-deployment proof.