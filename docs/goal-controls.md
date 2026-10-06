# Live user goal controls and session tasks

A **goal** is the overall objective, completion criteria, constraints, and
continuation policy. **Session tasks** are individual work items and their
statuses. They share an append-only journal, not a lifecycle: completing all
tasks does not complete the goal, and goal edits/clears do not remove tasks.

## User commands

These commands work while the agent owns the session and is processing:

| Command | Effect |
| --- | --- |
| `/goal show` | Show only the objective, criteria, status, and budget. |
| `/goal edit` | Open the current goal text in a prefilled multiline editor. |
| `/goal edit <objective>` | Edit the objective; preserve a pending answer review. |
| `/goal override [objective]` | Explicitly dismiss review and reopen the goal, optionally replacing its objective. |
| `/goal budget <positive tokens>` | Change the cap without resetting spent tokens. |
| `/goal budget none` | Explicitly remove the cap. |
| `/goal pause` | Stop goal continuation. |
| `/goal resume` | Resume without bypassing review or an exhausted cap. |
| `/goal done` | Record the human's explicit completion decision. |
| `/goal clear [reason]` | Remove the goal, preserving all tasks. |
| `/tasks [list]` | Show work items, including done/blocked/cancelled tasks. |

In the `/ask` answer-review prompt, type `/` to reach commands without accepting
the answer or granting tool approval. In the text editor, arrows/Home/End move the
cursor, typing/Backspace edits in place, Enter adds a line, and paste supports
multiline text. **Ctrl+S saves; Esc discards.** Opening/typing does not alter or
pause the live goal. Concurrent goal changes retain your draft instead of silently
overwriting them; live token accounting alone does not cause a conflict.

In the TUI, saving a changed goal cancels the old turn and resumes an active goal
with its updated objective after ownership
returns. It cannot undo tools that already ran. Overrides retain token usage;
an exhausted budget stays enforced until you explicitly raise/remove it.

## Authenticated user/admin API

The existing `GET` and `POST /api/session/{id}/goal` endpoints use the existing
authentication/authorization layers. Read the current `goal.id` and `updatedAt`,
then submit `goalId`, `updatedAt`, and `action`. Editable fields are `objective`,
`successCriteria`, `forbidden`, and `tokenBudget` (JSON `null` removes the cap).
Human actions include `override` and `force_complete`. Override accepts a stale
revision only for the same goal identity; it cannot accidentally edit a replacement
goal. HTTP changes are persisted immediately and read at model/goal boundaries;
they do not revoke a tool operation already in flight.

## Model tools

Use `create_goal`, `get_goal`, `edit_goal`, and verifier-gated `update_goal` for
goals. `session_task` advertises only `task_add`, `task_status`, and tasks-only
`list`. Its old explicit goal action names remain compatibility aliases, not
the advertised task workflow. Model `edit_goal` calls cannot invoke the human
override or force-completion actions, including by bypassing the JSON schema.

Configure verifier selection and inspect harness-owned model identity through the [verifier model API](api/verifier-model.md).