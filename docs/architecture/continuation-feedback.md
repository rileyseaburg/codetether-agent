# Continuation feedback and evidence preservation

The repeated-inspection incident in session `32e98b82` exposed two context-loss
paths. Corrections entered in the approval UI were retained only as tool errors,
while continuation summaries used search-ranked excerpts of the transcript.
Large documentation reads could displace the short corrections and outcomes
needed to proceed. Prefix-only summary truncation could then remove next steps.

## Runtime behavior

- An interactive denial carries its original feedback separately from tool
  output. Publication appends the denied tool result, then a pinned user message
  identifying that call, before saving. Ordinary tool metadata cannot populate
  this trusted feedback channel. Pinned sources use the existing persistence,
  explicit supersession, and context-budget enforcement mechanisms.
- Older sessions receive a budgeted reference to structured denial records in
  the loaded transcript. These references retain absolute source indices and
  tool provenance; they are **not** promoted to authenticated user instructions.
  The original database and messages are unchanged.
- Continuation summarization packs transcript turns in source order with a
  shared 64,000-byte ceiling. Short turns retain all text; long turns retain
  marked head/tail excerpts. Search and code-analysis requests continue to use
  retrieval. This is a bounded representation, not lossless transcript replay.
- Oversized summaries share space across Markdown sections instead of keeping
  only the prefix. The summary policy version invalidates older derived notes
  without deleting their source history.
- Continuation briefings prioritize observed callable interfaces and invocation
  syntax, corrections, attempts, and the next action. A list of inspected file
  names is insufficient working memory. Repeated status/provenance prose must
  not displace the concrete facts needed to use a previously inspected helper.
- The proactive producer and reader share a policy-versioned cursor/view name.
  A new summary policy therefore replays source rather than inheriting an old
  acknowledged cursor and permanently losing rejected summaries. Source edits
  rewind versioned RLM cursors to the same 16-message boundaries as legacy RLM
  cursors. Previous policy views remain intact unless their source is edited.
  A missing native RLM cursor starts at an aligned boundary at least 128
  messages from the end, so a policy upgrade rebuilds recent working memory
  without sending a multi-thousand-message archive through the model first.
  Existing cursors never fast-forward; older source remains retrievable.
- Prepared summaries may overlap retained raw turns when they also cover omitted
  history. A summary crossing the recent-tail boundary is no longer discarded
  wholesale. Raw turns remain intact, a summary spanning multiple pin gaps is
  emitted once, and only its omitted-history coverage counts toward the raw
  cost it replaces.
- A durable review clock runs independently of call/result novelty. Every 15
  completed non-wait results, the prompt loop inserts developer guidance to
  reconcile the current task, corrections, completed results, and next action.
  It reports observed failures, exact repeats, and source positions without
  treating novelty as task progress. Successful waits do not advance this clock;
  failed waits do. Human input resets it, but automatic continuations, new
  command strings, and session reloads cannot postpone it. This is a periodic
  review, not a semantic stagnation detector or an authorization grant.

The earlier overlap-detector prototype was removed. Content similarity alone
does not prove stagnation or task completion. These changes repair evidence
delivery rather than force tool execution or infer authorization.

## Validation

Focused tests cover publication ordering and spoofed metadata, pinned feedback
after a native session reload, chronological synthesis input under large tool
output, and preservation of late summary sections. The opt-in `feedback_replay`
test reads the incident database without writes and checks three original
corrections in final provider requests at three transcript boundaries and three
input budgets. It uses no provider calls and performs no funnel operations.

These deterministic checks validate request contents and runtime state, not
subsequent model behavior. The opt-in `continuation_behavior_live` harness also
captures decisions from the incident model with native prompts and tool schemas.
Its nonempty-call assertion is a capture check, not a semantic success criterion.
The acceptance rubric and all failed captures are retained under
`artifacts/session-continuity-32e98b82/`. Current one-turn captures still reread
helpers; the patch must not be described as a proven end-to-end loop fix on the
basis of unit tests or a successful install. Follow-up captures supply manually
reviewed read-only observations to test retention across another turn. At the
recorded late-stage stall (source boundary 4790), the normal-budget replay reads
one missing authentication helper and then proposes the authenticated dev
inventory operation. Its extracted JavaScript passes a non-executing syntax
check. The earlier boundary still investigates contracts. See the artifact
`behavior-assessment.md` for budget assumptions and these distinct outcomes;
no captured business operation was executed.
