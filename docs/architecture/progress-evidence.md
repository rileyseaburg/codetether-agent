# Progress and repeated-result evidence

`ProgressLedger` is stored in session metadata, alongside the canonical
conversation. It pairs tool calls with results by call ID and retains at most
128 pending calls and 128 observations. Records contain input/result hashes,
execution failure status, repetition counts, and absolute result positions;
they do not retain raw argument or output payloads.

An attempted call is not a completed observation. The foreground loop and
worker record the runtime outcome before result formatting or RLM routing can
change the displayed text. Generic replay uses its explicit-status transcript
encoding. The later transcript append consumes no second observation for an
already recorded result. Worker transcripts also preserve execution status,
original output whitespace, and image attachments for checkpoint recovery.

New call/result combinations are **observations, not proof of task completion**.
Different read-only queries can contribute useful evidence without any file
edits. Identical outputs from identical calls, including cycles through
previously observed variants, accumulate a repeated-result count. Successful
wait/poll operations do not count as stalled execution; repeated failed waits
are compared like other failed outcomes. At the threshold the
foreground loop offers nonterminal guidance: investigate the next uncertainty,
adjust the approach, or continue legitimate waiting with backoff. It does not
tell the model to end the task or require a file write.

Only an unchanged edit that returned the same explicit failure three times
can be rejected before execution. Successful edits, shell calls, reads, and
waits do not receive that rejection. Changed arguments, fresh observations,
or new human direction reopen retry consideration. Synthetic repeat-guard
responses are not new execution evidence and cannot reset their own guard.
Runtime goal/checkpoint continuation prompts preserve the evidence epoch.
An independent conversation fork starts a new ledger.

The old per-turn edit-attempt counter and file-write counter are no longer in
the production prompt loop. `RepeatGuard` remains as the standalone legacy
compatibility API; production uses `ProgressLedger`. Ordinary goal-turn resets
leave session metadata intact, and short-window reloads recover the same ledger.

This is bounded evidence tracking, not a semantic grader. New output does not
by itself prove that the user's goal advanced or is complete. Requirement-level
verification remains necessary. Older sessions without a ledger begin with an
empty one; unrecorded historical actions are not invented.
