# Shared session runtime ownership

The objective is one execution-control owner for steering, pausing,
cancellation, and continuation across the TUI, CLI, and A2A workers. A frontend
should translate user input and render results. It should not independently
decide whether a correction stops the task.

## Ownership and dependency direction

| Layer | Responsibility | Allowed dependencies |
| --- | --- | --- |
| `codetether-goal-core` | Durable goal/task events, replay, checked human/model edits, review and budget invariants | Storage/serialization libraries; no agent or frontend |
| `codetether-session-runtime` | Process-wide control ownership, run leases, executor handoff, cancellation state, steering inboxes and completion boundaries | Runtime primitives; no TUI, provider, tool registry, or agent crate |
| Agent session runtime adapters | Session identities, journal paths, transcript ingestion, IPC, activity notifications, execution-policy integration | Shared runtime and goal core; application services |
| TUI, CLI, HTTP, worker adapters | Input transport, presentation, frontend lifecycle and task envelopes | Session runtime interface |
| Provider/tool execution | Model requests, tools, approval checks, execution results | Runtime decisions; never frontend-specific controls |

Goal completion and model-message completion remain distinct. An unfinished
active goal can request another model turn. An explicitly paused goal, answer
hold, or exhausted budget must not be resumed by a model's prose.

## Evidence and extraction order

1. **Extract shared mechanisms without changing behavior.**
   `src/tui/app/session_runtime/active_turn.rs` imports the extracted
   `ActiveTurn` for compatibility tests. Session steering uses the crate's generic
   `Inboxes`. Cancellation before executor attachment remains pending, rejected
   attachments cannot steal ownership, and an empty completion drain closes
   the inbox atomically. Application activity notifications stay in adapters.
2. **Consolidate application adapters under session runtime ownership.**
   Goal runtime operations now live under `session/runtime/goals`; steering
   and IPC live under `session/runtime/steering`. The old
   `session/tasks/runtime` and `session/helper/steering` paths are compatibility
   exports to the same implementations, not duplicate runtime state. The
   session facade's public exports have their own module to keep its module
   declarations within the repository's file-size limits.
3. **Integrate both execution loops.**
   TUI and CLI use `session/helper/prompt_loop`. A2A workers use
   `a2a/worker/session_steps.rs`. Workers need the same steering boundaries,
   hold checks, cancellation owner, goal accounting, and continuation decisions.
   These model loops now enter `session/runtime/execution.rs`, using the same
   process-wide `ControlPlane` and generation-scoped run leases. TUI-local state
   selects the active lease instead of implementing another cancellation state
   machine. `boundary.rs` checks cancellation and durable holds before steps and
   tool dispatch. `continuation.rs` routes final messages and reopens steering
   for continuing goals. Worker approval policy, hard task step budgets, and
   output callbacks remain in worker adapters.
4. **Correct input semantics at the shared boundary.**
   Ordinary corrections must steer ongoing work. An explicit side-question
   request may enter answer review. Preserve durable hold identities, human
   authority, budget limits, and tool approvals. Test the actual frontend
   dispatch as well as the shared policy. Ordinary input now bypasses the
   side-question parser; only explicit `/ask` enters that parser's hold path.
   Plain follow-up questions remain context-only while a review hold is already
   open. Existing held-goal `/ask` continuation behavior is retained.

## Swarm and delegated execution

`a2a/worker/task_agent.rs` has a second branch for swarm agents. That branch
uses `swarm_policy.rs` and `swarm_policy/swarm_policy_run.rs`, bypassing the
ordinary model loop. This path now claims the same `RunGuard` and attaches a
`ControlHandle` to `SwarmExecutor::with_control`. `src/swarm/control.rs` is a
compatibility alias; it no longer owns a separate cancellation token or pause
flag. Both flags live in the shared crate's generation-checked control table.

The streaming worker runs orchestration inline. The shared monitor keeps the
canonical session inbox and durable hold checks live while delegated work runs.
On cancellation it signals the shared control and awaits executor cleanup.
Local-stage destruction aborts remaining child handles. The Kubernetes stage
stops scheduling and cleans residual pods on cancellation. A correction also
ends the remote stage, because an already running pod cannot receive this
process's in-memory steering; the parent then reviews partial results.

Standalone swarms reserve a shared lease and release it on the last handle's
drop. Worker handles borrow the existing owner instead. Local child jobs
explicitly inherit the handle and independently read a broadcast log of accepted
human input. Their model and tool boundaries honor shared cancellation, pause,
and durable holds. Child provider tokens are charged to the parent goal, and
the monitor records elapsed time once for parallel work. Ordinary session
executors also propagate their handle into directly spawned `swarm_execute`
children; accepted steering in that tool path is applied at the next parent
conversation boundary.

After saving swarm output, unfinished goals and steering return to the ordinary
session loop with the original worker approval policy. This preserves completed
work and uses normal verified goal completion rather than restarting the swarm.
The outer worker retains its run lease throughout that handoff. An
`ExecutionScope` borrows it only through explicit task-local propagation;
independent callers still fail acquisition before touching session history.
The inbox stays open between executor phases and during goal-journal writes.
Only a final decision with no continuation closes it atomically; cancellation
and failure close it after draining accepted input.

Swarm decomposition checks the same readiness gate before provider setup and
again before requesting a plan. Cancellation also interrupts setup. Stage
transitions call that gate directly instead of maintaining a second pause loop,
so durable holds apply there as well as inside each child agent.

## Verification required before completion

- Shared runtime tests cover cancellation during handoff, stale cancellation,
  session isolation, FIFO delivery, cleanup, and concurrent finalization.
- Existing session/TUI tests continue to cover images and activity cleanup.
- Both execution loops use the same control owner, including worker steering,
  pause/cancellation, and active-goal continuation tests with mocked providers.
- Corrections reach the working agent without cancellation or answer review;
  explicit side questions retain their documented behavior.
- Crate manifests and dependencies reflect implementation ownership; public
  compatibility paths resolve; changed files meet repository size limits.
- No broad local build is required; run focused tests and leave full build/type
  coverage to CI as required by `AGENTS.md`.

## Validation without Cargo

The current implementation was verified on 2026-10-17 using native `rustc`
and `rustdoc`, with no Cargo commands. Cached dependency artifacts were reused;
the runtime crate and the application's library test runner were compiled from
the current source. The runtime dependency was explicitly replaced with its
freshly compiled artifact before compiling the application runner.

Results:

- Runtime crate: **10 unit tests passed**.
- Runtime documentation: **6 runnable examples passed**.
- Application: **88 focused regression tests passed**, with no ignored tests.
- Native Rust formatting, whitespace validation, and changed-file line limits
  passed. The runtime crate's source files also satisfy the 50-code-line limit.

| Requirement | Current evidence |
| --- | --- |
| One control owner with safe handoff and stale-handle rejection | Core control, observer, cleanup, and turn tests |
| Ordinary corrections continue active work | Actual TUI Enter dispatch and worker final-response steering tests |
| Holds, cancellation, and budgets remain authoritative | Worker pause/review, in-flight cancellation, step-budget, and token-budget tests |
| Delegated work uses parent controls | Child cancellation, steering, token accounting, and spawned-handle tests |
| Swarm-to-worker handoff retains ownership | Actual nested worker-loop and duplicate-worker history-preservation tests |
| Continuation does not lose racing input | Test holds a goal-journal append pending while submitting a correction |
| Cancellation waits for child cleanup | Shared monitor cleanup test and local-stage abort-all test |
| Existing review and image behavior survives extraction | Answer-review, TUI lifecycle, and delegated tool-image tests |

The application test runner used the default and TetherScript features. This
was focused verification, not a release build or a live Kubernetes deployment.
Its compiler reported existing warnings. The first native compile attempt
stopped because package-description metadata was missing; supplying the
manifest metadata directly corrected the invocation without a source change.

Raw logs, compiler arguments, package metadata, and source SHA-256 snapshots
are retained in `artifacts/session-runtime-native-20261017.Psv2ox/`. Source
hashes were rechecked after compilation and test execution. The earlier
verification blockage is resolved; the zero-Cargo instruction remains intact.

## Broader workspace consolidation

The separate vector-store scaffold has now been completed as part of the broader
source-to-crate consolidation. Storage and local embeddings live in
`codetether-vectordb`; provider/backend adapters remain under `src/vectordb`.
See [the workspace inventory](src-crate-consolidation.md) for its verification
and the remaining owners. Completing this session runtime does not complete
the broader workspace goal.
