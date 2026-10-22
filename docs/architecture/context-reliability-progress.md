# Context reliability implementation progress

All seven improvements identified in the context, caching, goals and long-horizon
audit are implemented and verified. This file retains the incremental history;
`context-reliability-audit.md` records the final requirement-by-requirement audit
and explicit limits on live-provider claims.

## Implemented in the first increment

- **Checkpoint evidence:** Tool attempts are matched to results by call ID.
  Only an explicit success envelope populates `completed_actions`. Failed,
  pending, and returned-with-unknown-status actions remain distinguishable.
  Complete loaded user text and absolute source indices are retained separately
  from assistant statements. Legacy checkpoint action lists are labeled
  unverified. Unicode truncation no longer slices through UTF-8 characters.
- **Journal replay:** The runtime cache folds only newly appended bytes when
  file identity is stable. Replacement, truncation, and changed same-length
  files trigger a cold fold. The cache is bounded with LRU eviction and uses
  per-journal locks. Malformed events and incomplete tails are reported instead
  of skipped; execution gates reject unreadable state and prompt composition
  reports a governance error. Writers synchronize event data to disk.
- **Prompt layout:** Goal snapshots are marked during composition and moved
  into a runtime-data message near the current user input during shared request
  assembly. Stable instructions and earlier history remain unchanged as goal
  counters change. A wire-level test exercises goal-journal updates, prompt
  composition, request construction, and Codex encoding. It also exercises the
  short `continue` input that previously selected an older active-task anchor.

## Second increment: durable source storage

The current work adds complete pinned-source records, text hashes, speaker roles,
and explicit whole-turn replacement links. Database version three rebuilds the
index from canonical messages rather than preserving truncated legacy excerpts.
Working-window eviction retains these records, and incremental derivation
reserves their mandatory header before selecting optional history. Text-only
pins covered in that header avoid duplicate mandatory reservation; attachments
continue to protect their original message.

Pin/unpin now translates absolute transcript indices into the loaded window.
Whole-turn supersession requires a later user source, validates both ends before
mutation, and rejects stale retries that would reactivate an unpinned or
subsequently replaced source. See `durable-constraints.md` for the contract.

This increment does **not** complete fine-grained partial-correction semantics.
Review also identified missing refresh hooks outside the TUI. The worker response
path appended assistant messages and tool results without immediately saving
them; copying the TUI reload alone would have dropped those pending entries.

Second-increment verification so far: 163 unit/integration tests and 116 doc
tests passed across context policy, pinning, dispatch, incremental derivation,
and session storage. Two optional tests remain ignored. The final `context_pin`
runtime-entry rerun subsequently passed all four tests and its doc example.
The first native run failed when it compiled against a worker interface changed
while that run was active; the replacement native run is listed below. Initial
core-suite output is in `artifacts/context-reliability-constraints/`.

## Third increment: reconcile externally edited sessions

- The TUI, worker, and generic agent replay paths now save pending transcript
  data before context-edit tools and refresh the session after successful edits.
  They share the tool classification and the application's reload method.
- Generic sequential replay records each tool result immediately. A later
  context edit therefore persists preceding results instead of losing them
  when it reloads the session. Later tools see the refreshed context.
- Save failure prevents tool execution. Refresh failure retains the completed
  tool result and stops later calls in the same worker or agent response.
  Worker errors explicitly distinguish a completed edit from a failed refresh.
- Read-only parallel batches and denied tools do not invoke these write hooks.

Focused verification passed 61 unit/integration tests and 99 doc tests across
worker core and agent replay. Existing 23 ignored worker examples remain ignored.
Native verification passed 15 session-store tests (one incident-fixture test
ignored) and both worker image/denial tests. The earlier interface mismatch is
resolved. Formatting, whitespace, and the application file-limit check passed.
The new shared context-edit classifier's doc example and the native worker
`tool_call` doc test also passed. This increment totals 78 unit/integration tests
and 101 doc examples passing across focused and native checks. The existing
ignored examples and incident-fixture test were not run. No verification
process from this increment remains pending. Evidence is under
`artifacts/context-reliability-refresh/`.

## Fourth increment: summary provenance

New summaries record complete source-message hashes, the shared summarizer
policy version, the producer path, and the requested model. Incremental
derivation and its shared finalizer validate source identity before using a
summary. A stale local entry cannot overwrite a valid prepared entry.
`context_summarize` also validates its early cache-hit path and newly produced
nodes. Background inputs include source, policy, and model identity.

Legacy summaries remain readable, but unprovenanced or stale entries are
excluded from working projections. Unchanged ranges remain reusable across
appends, SQLite loading, and window rebasing. See `summary-provenance.md`.
Verification passed 172 distinct unit/integration tests and 61 doc examples
across the focused suite, SQLite/window regressions, explicit summary-tool
cache checks, and native session-store tests. The optional real-session replay
and local incident fixture were not run. Native tests retain 22 existing
warnings. Formatting, whitespace, and the application line-budget check passed.
Evidence is under `artifacts/context-reliability-summaries/`. These checks do
not measure live summary quality or provider cache-hit rates.

## Fifth increment: canonical checkpoint recovery

Checkpoint creation now uses absolute message counts and streams a consistent
canonical SQLite snapshot, including evicted user corrections and tool attempts.
The fold rejects missing records and header-count mismatches. Resuming reconciles
canonical history even after same-length edits. The model-facing checkpoint is
bounded, while complete source evidence remains persisted. Regression coverage
passed across saturated windows, cold reloads, concurrent database
edits, missing records, and both CLI resume loops. Saves now commit canonical
metadata before atomic, synchronized sidecar replacement; failed metadata saves
restore in-memory state. See `checkpoint-recovery.md`.

This increment passed 19 distinct unit/integration tests and 21 doc examples.
Native checks cover snapshot isolation while a second SQLite connection edits a
record, missing-record rejection, and recovering an evicted tool attempt whose
result is still in the later transcript. The resume fixtures cover three
successive attempts with an unchanged working-window length and same-length
user corrections. Atomic-sidecar tests include concurrent readers and injected
metadata/publication failures. Formatting, whitespace, and the application
line-budget check passed. Evidence, including initial failed checks and their
successful replacements, is in `artifacts/context-reliability-checkpoints/`.

## Sixth increment: durable clearing and progress evidence

Checkpoint clearing now commits an explicit tombstone before deleting its
fallback sidecar, so interrupted cleanup cannot resurrect the checkpoint.
The persisted `ProgressLedger` records actual tool outcomes across prompt/goal
continuations and reloads, with bounded hashed records and absolute references.
Foreground and worker paths capture raw outcomes before formatting; worker
history retains status and images. The production loop no longer treats a shell
invocation as a file write or counts an attempted edit as a failed execution.
Repeated outcomes produce nonterminal guidance; only unchanged, explicitly
failed edits receive the targeted retry guard. Forks begin a new ledger.
Verification passed 66 distinct unit/integration tests and 68 doc examples.
This includes cold native reloads, worker status/image preservation, repeated
failed waits, raw outcomes containing nested feedback headers, and rejection
checks across continuations. One pre-existing doc example remains ignored;
native tests retain 22 existing warnings. Formatting, whitespace, and the
application line-budget check passed. Raw logs retain the initial misplaced-hook
compile failure and line-budget failure alongside their successful replacements.
Evidence is in `artifacts/context-reliability-progress-evidence/`.
See `progress-evidence.md`.

## Seventh increment: shared request layout and fresh runtime snapshots

The prepared foreground builder now shares system/history assembly with the
worker and generic-agent context path. Runtime and optional context placement
preserves completed tool history, with fresh user input last when present.
The foreground refreshes accounting during request preparation, and repeated
composition replaces old snapshots rather than accumulating them. Explicit
journal-error snapshots use the same data frame and disappear on recovery.
Tests cover actual journal updates, both request paths, five wire-format
families, long tool runs, short follow-ups, model switches and budget failures.
Verification passed 43 unit/integration tests and 21 doc examples. This includes
the native foreground adapter, both request-construction paths, six encoding
paths (Codex HTTP/WebSocket, Anthropic, Bedrock, Google and OpenAI chat), and
twenty tool-free continuation/restart cycles that leave evidence counters and
failed-edit guards intact. Scoped formatting, whitespace and application
file-limit checks passed. The initial insertion-order failure is retained
alongside its successful frozen-source replacement. Evidence is in
`artifacts/context-reliability-request-layout/`. No live-provider cache usage
was measured. See `request-cache-layout.md`.

## Eighth increment: exact partial corrections

`context_pin correct` now records source hashes and exact UTF-8 spans linked to
later user messages. Multiple disjoint changes preserve every unrelated byte.
The original may be evicted; the replacement is pinned in the same transaction
as correction metadata. Failed saves roll back staged changes. Shared context
headers apply validated replacements across derivation strategies and retain
the original text with an explicit warning if a stored link becomes invalid.
Canonical source text is never rewritten. Verification passed 152 distinct
unit/integration tests and 59 doc examples, including forced compaction,
repeated native cold reloads, stale/overlapping/Unicode spans, and save rollback.
Native checks retain 22 existing warnings. Formatting, whitespace and application
file-limit checks passed. Evidence is in
`artifacts/context-reliability-partial-corrections/`; see `durable-constraints.md`
for the contract and current window limits.

## Ninth increment: correction-aware forks

Edited-prefix, interactive and detached forks preserve pin classifications and
rebase whole-source and partial-correction links to child message indices.
Forking before a replacement restores the original pin, while detached tails
discard links to source messages outside the selected window. Parent state is
unchanged. Seven unit/integration tests and six doc examples passed, including
native persisted-tail rebasing and continued cold-reload preservation. Formatting,
whitespace and application file-limit checks passed. Native tests retain 22
existing warnings. Evidence is in `artifacts/context-reliability-fork-corrections/`.

## Tenth increment: bounded historical pin edits

Pin operations now prepare at most two canonical sources by absolute index,
including originals and replacements outside the loaded window. Sparse page
changes commit atomically with session metadata under the existing revision
guard. They update the full-source index and immutable audit records, invalidate
affected projections and leave tool outcomes and unrelated transcript rows
unchanged. Both full and partial corrections use the same tool execution path
as resident messages. Verification passed 28 distinct unit/integration tests and
69 doc examples. Coverage includes native historical pin/unpin/supersession,
partial corrections with both sources evicted, concurrent revision rejection,
second-source rollback, bounded audit writes, and preservation of blob content
and tool outcomes. Formatting, whitespace and application file-limit checks
passed. One pre-existing documentation example remains ignored, and native
checks retain 22 existing warnings. Evidence is in
`artifacts/context-reliability-indexed-pins/`.

## Final increment and completion audit

The native combined evaluation now passes three compaction/restart phases with
successive corrections, preserved failed/pending actions, human redirection,
model changes, checkpoint save/reload/clear, selective summary invalidation and
stable consecutive request encoding. Four rejected local completion attempts
exercise provider-outage fallback; no live-provider request is sent.

This evaluation found two further issues and verified their fixes. Cached
journals retain bounded file handles so inode reuse cannot masquerade as an
append. Indexed vector mutations retain the actual changed suffix, preserving
unaffected prefix summaries instead of invalidating the full loaded window.
Atomic-replacement races and indexed scalar/range edits have dedicated tests.

The final focused regression run passed **424 unit/integration tests and 381 doc
examples**, plus the separate native combined test (**425 tests total**). One
optional integration case and 24 existing doc examples remain ignored. Formatting,
whitespace and application line-budget checks passed. Raw results and source
fingerprints are under `artifacts/context-reliability-final/` and
`artifacts/context-reliability-combined/`. Initial failures and the interrupted
noncanonical-locator test are preserved. All seven requirements are covered in
`context-reliability-audit.md`. Live cache hit rates, latency and cost savings
remain unmeasured; no deployment or publication was performed.

## First-increment verification record

Checkpoint regression tests and its ten doc examples passed before the later
shared-provider changes. Journal core/runtime unit tests passed; an existing
human-budget-edit test was corrected to use the human edit entry point, and a
stale extracted-crate doc import was corrected. The first wire-level cache test
exposed runtime state being inserted before old history for a short `continue`
input; request assembly was corrected to use the latest user-message boundary.

Final verification: **238 unit/integration tests and 160 doc tests passed**
across the five affected crates. The documentation reruns corrected stale
extracted-module imports and one example whose arguments were below the
documented deduplication threshold. No runtime behavior was weakened to make
those examples pass. Scoped Rust formatting, whitespace, and the application
file-limit check passed.

Verification commands for this increment:

```sh
cargo test -p codetether-goal-core -p codetether-goal-runtime \
  -p codetether-run-checkpoint -p codetether-context-policy \
  -p codetether-provider-core --offline
git diff --check
./check_file_limits.sh
```

The aggregate test command passed all unit/integration tests, then exposed
pre-existing documentation failures. After correcting the examples, doc tests
passed in two targeted reruns. Raw outputs, including the initial failures,
and source hashes are retained under `artifacts/context-reliability-first-increment/`.
Those first-increment results were not sufficient by themselves; the later
increments and final audit above cover the remaining requirements. Full
application build/type coverage remains with CI, as required by the repository
instructions.
