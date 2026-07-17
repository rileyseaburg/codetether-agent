# First-party mDNS collaboration proof

Run on 2026-07-17 with:

```text
CODETETHER_MDNS_PROOF_DIR=artifacts/mdns-first-party-proof-2026-07-17 \
cargo test --test a2a_mdns_process_e2e -- --nocapture
```

Result: one test passed, zero failed; the test executed three sequential rounds.
Each round launched two independent `codetether spawn --name ...` OS processes
using the CLI defaults, an empty `CODETETHER_A2A_PEERS`, no `--peer`, and no
`--hostname` override.

Every round asserted reciprocal mDNS discovery, reciprocal authenticated
automatic introductions, a successful first-party Agent Tool list/message turn,
401 responses for a missing token and the other peer's control token, and a 200
completed response for the target peer's own control token. The six logs also
record `bind_addr=0.0.0.0`. The test writes a `round-N.passed` manifest only after
all assertions pass and both child processes have been stopped and reaped.

The first pre-fix three-round run failed in round 3. Its newly started peer reused
the legacy ledger endpoint `http://192.168.50.101:43475`, so endpoint-only dedup
silently suppressed its introduction. The fix keys first-party introductions by
the endpoint plus hashes of both processes' mDNS capability identities. The
ledger retains the legacy endpoint and the new hashed identities; it was not
cleared to make this proof pass.

Additional focused regressions passed:

- `cargo test --test a2a_mdns_collaboration_e2e -- --nocapture`: 1 passed.
- `cargo test --test a2a_intro_e2e -- --nocapture`: 2 passed.
- `./check_file_limits.sh`: passed.
- `git diff --check`: passed.

`SHA256SUMS` authenticates the six raw process logs and three pass manifests.
