# Answer review while a goal is running

A TUI question interrupts goal work without granting tool permissions. Plain
chat input and `/ask <question>` create a durable answer-review hold when the
session has an unfinished goal. Without a goal, normal input handling applies.

## Interaction

1. Submit the question. The goal becomes paused and the active local turn is
   asked to cancel.
2. Once that turn returns ownership of the session, the question is answered
   using the session context, without tools or saved conversation messages.
3. The satisfaction prompt offers **Yes** and **No**, with **No** selected by
   default. Use Left/Right or Tab to select, then Enter; Y/N also decide.
4. **No** keeps the goal paused. Submit a follow-up question to obtain another
   answer and a new satisfaction prompt.
5. **Yes** restores the goal's prior status and allows continuation. An exhausted
   token budget remains budget-limited instead of becoming active.

Escape means No. Modified acceptance keys do not accept the answer. Scrolling
and the existing quit shortcuts remain available.

## Boundaries

- The hold is independent of `ask`, `approve`, and `full` access modes. Accepting
  an answer does not approve a pending tool; approving a tool does not release
  the answer-review hold.
- Runtime status updates cannot resume, complete, or block a held goal.
  Accounting updates still record usage. Queued prompts and direct runtime
  submissions are gated until the hold is released.
  Model-facing goal replacement is refused, and replay ignores non-user
  replacements that were already in flight when the hold began.
- Question and decision events carry both goal and review identities. Stale
  answers and decisions cannot release a replacement question or goal.
- The append-only task log retains the hold and answer readiness across restart.
  Questions and review decisions are saved there; "ephemeral" means they are
  not added to the saved conversation, not that the question is unlogged.
  Abandoning a goal through the user's explicit clear command or replacing it
  removes its obsolete review; this is not accepting an answer.
- Cancellation is cooperative. It cannot undo an external action already
  performed or retract a request already sent.

Focused tests cover replay, stale decisions, budget preservation, keyboard
choices, runtime rejection, and mocked local answer delivery and rendering.