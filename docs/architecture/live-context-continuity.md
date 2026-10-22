# Live session context continuity

Context continuity must preserve what the user asked for, what the agent has
verified, and what remains uncertain. Persisting the transcript alone does not
ensure that the next model request contains those distinctions.

## Failure reproduced from a long TUI session

A reviewed session contained 513 messages, 31 prepared RLM summaries, and no
explicitly pinned constraints. The original request asked for CLI/database
investigation and an activation goal. Later assistant messages repeatedly
rediscovered the same deployment architecture and pursued builds and pushes.
The user subsequently corrected that approach.

The prepared summaries changed an unresolved inference about deployment into
an established prerequisite, and eventually described inferred authorization
as consent. This session had not reached the 1,000-message working-set limit:
its immediate problem was summary fidelity and continuation provenance, not
restart loading or working-set eviction.

## Corrections

- The active RLM engine now routes `proactive_session_context` and
  `summary_index` through the continuation query, alongside `session_context`
  and `context_reset`. Previously it replaced their summary-specific request
  with a generic request for important information. The target token count is
  included in the synthesis query instead of relying only on output truncation.
- Continuation summaries distinguish user requests and corrections, assistant
  proposals, observations, and unverified conclusions. A partial transcript
  must identify missing original instructions rather than infer them.
- Automatic goal reminders identify themselves as runtime-generated messages.
  A stored objective may be an assistant paraphrase; it does not expand user
  authorization or override subsequent corrections. Turn instructions also
  preserve this distinction and encourage reuse of verified progress.
- Incremental derivation merges the live summary cache with prepared background
  summaries, preferring local summaries for identical ranges. Successful
  `context_summarize` calls refresh the live session before publishing their
  tool result, using the same persistence boundary as `context_pin`.
- When incremental compaction is needed, a separate user-request reference
  preserves all loaded user text when it fits; otherwise it keeps bounded
  excerpts from the earliest request and up to seven latest user messages.
  Automatic goal reminders are excluded. Its estimated
  cost is reserved before selecting history, capped at 2,048 tokens and one
  quarter of the message budget after system/tool overhead. Long excerpts
  preserve their beginning and end with an explicit omission marker. Requests already under budget are
  unchanged; extremely small budgets can omit the reference. This is direct
  source evidence alongside summaries, not another model-authored summary.
- Working-set eviction rebases wholly retained summary ranges and preserves
  their LRU ordering. It discards summaries overlapping removed messages,
  instead of clearing every cached summary on each append after 1,000 messages.
- Bounded manual-summary reads filter out-of-window ranges before applying the
  result limit, so unusable recent rows cannot hide usable earlier summaries.
- The final overflow clamp can remove older complete assistant/tool exchanges
  after the active request when the nominal recent-message window is itself
  too large. It preserves the active user instruction and newest exchange,
  removes call/result groups together, and updates residency and dropped-range
  metadata. If those mandatory entries alone exceed the budget, the existing
  over-budget fallback still applies.

## Regression coverage and limits

The RLM test captures the actual synthesis request for a transcript containing
a user task, an assistant-inferred build/push prerequisite, and a user correction.
It checks that the continuation query, provenance requirements, token target,
and both source statements reach the model for all four context-summary routes.
The provider is scripted: this tests routing and evidence delivery, not whether
a live model will always follow the instructions.

The incremental projection tests also recreate a long live conversation with
an assistant-inferred build/push prerequisite, a user correction, and an
automatic goal reminder. They verify that the direct user excerpts survive
compaction, the reminder is excluded from the reference, message metadata
remains aligned, and the final request fits the configured budget.

Other focused tests cover local-summary selection under a token budget, 101
successive working-set evictions, LRU order after serialization, and bounded
summary loading. Historical prepared summaries are not rewritten by these
changes. A live-provider replay and an interactive TUI run remain useful
follow-up validation; passing deterministic tests does not establish that all
long-horizon reasoning drift is solved.

Initial validation: 44 focused tests passed across summary-index
behavior, incremental derivation, rolling storage, the RLM engine/router,
goal runtime, and the new cache API's doc example. Rust formatting,
`git diff --check`, and `check_file_limits.sh` passed. No full application
build, live-provider replay, or deployment was run.

Follow-up validation of direct-user references and active-window overflow:
all 126 context-policy tests and all 13 incremental-derivation tests passed.
This includes a 241-message reproduction that initially exceeded its 20,000
token budget; after removing an older complete exchange it retains the direct
user reference and fits the budget without changing canonical history.

The reviewed journal also marked the model-expanded goal as `user_provided`
while leaving its source turn and source text hash empty. New model-created
goals now record the latest saved user-message index and SHA-256 digest when
available, excluding automatic goal reminders. An objective that matches the
source text is classified as `user_provided`; derived wording is `raw_turn`
with lower confidence, and missing evidence is `inferred` with zero confidence.
The source reader opens SQLite read-only and neither migrates nor creates a
session database. It checks at most the newest 1,000 record positions and 64
user messages. Model-supplied provenance fields are ignored.

`create_goal` and `get_goal` expose the recorded reference and flag missing
references, including those in old journals. If the objective has subsequently
changed, the original declaration's source is not attributed to the changed
wording. A source reference supports tracing the objective; it does not prove
that an expanded implementation plan was authorized. Existing journal entries
are not rewritten.

Provenance validation passed: three source-reader tests, two goal-tool tests,
one native goal-runtime test, one native goal-tool contract test, and five new
source API doc examples. Coverage includes spoofed model metadata, exact text
versus derived wording, missing/failed lookup, source retrieval through
`get_goal`, stale source invalidation after objective changes, known SHA-256
output, large blob-backed messages, and byte-for-byte preservation of the
read-only database. Existing unrelated browser/test dead-code warnings remain.

## Read-only replay of the supplied session

The optional `session_replay` integration test loads a consistent SQLite
snapshot using a read-only connection, including the actual tool messages and
existing RLM summaries. It runs incremental derivation and the same final
request builder used by the prompt loop without calling a model or executing
any tools. It uses a small explicit system prompt and excludes optional
recall/LSP context and the live tool catalog; it is a history-pipeline check,
not a full interactive TUI or provider evaluation.

The replay initially reproduced a missing earlier user correction at 16,000
tokens with the three-recent-message reference. The reference now retains all
short user messages that fit the reserved budget and uses the first-plus-seven
fallback only when needed. The 534-message snapshot then passed at all four
budgets, with estimated request sizes of 15,957 / 16,000; 24,063 / 32,000;
55,967 / 64,000; and 97,304 / 128,000. Aggregate results are written locally to
`artifacts/context-replay/report.json`; no transcript text is included. All 15
incremental-derivation unit tests also pass.

To replay another session without network calls:

```sh
CODETETHER_REPLAY_SESSION=/path/to/sessions/SESSION-ID.json \
  cargo test -p codetether-incremental-derivation --test session_replay -- --ignored --nocapture
```

## Final integration audit

- User-reference labels now include the native working-window offset. A
  retained message at local index zero in a window beginning at 1,500 is
  labeled 1,500, matching the absolute indices accepted by `context_browse`.
- All 17 incremental-derivation tests pass, including direct formatting and
  full derivation through the offset-aware session binding.
- The native agent test
  `keeps_active_instruction_but_leaves_old_tool_work_compressible` passes,
  compiling the actual TUI/session adapters with these changes.
- A fresh read-only replay captured 540 messages and passed all four budgets:
  15,276 / 16,000; 25,333 / 32,000; 62,899 / 64,000; and 66,689 / 128,000.
  The aggregate report replaces the earlier snapshot's report without
  storing transcript content.
- Rust formatting, whitespace checks, and the application file-limit check
  pass. Existing unrelated unused-code warnings remain.

This completes the identified context-retention and provenance repairs and
their local verification. No live model requests, deployment, or rewrite of
the supplied session was performed. Provider behavior beyond the tested
request construction remains outside these deterministic checks.
