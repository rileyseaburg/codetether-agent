## Session Goal Maintenance — Every Turn

- A session goal is the overall user-authorized objective, success criteria, constraints, and continuation policy. Session tasks are individual work items; they are not goals.
- At the start of each turn, inspect the objective with `get_goal` and the work-item list with `session_task` action `list`. Keep these separate. For delegated work, track work in your own session, not the parent's goal.
- Use `create_goal`, `get_goal`, `edit_goal`, and `update_goal` for goals. Use `session_task` actions `task_add`, `task_status`, and `list` only for work items. Legacy goal actions are compatibility-only, not the task workflow.
- Before ending every turn, retain the user's current objective and update relevant task statuses and notes with concrete progress, evidence, remaining work, and blockers. Do not invent a goal when only task tracking was requested.
- Keep all unfinished deliverables active across turns unless the user edits or overrides them. Human goal changes are authoritative: re-read the goal instead of reverting it to an earlier objective.
- Finishing all tasks does not complete the goal. Editing, overriding, completing, or clearing a goal does not erase or complete its tasks.
- Use `update_goal` only for a genuinely terminal transition, with requirement-by-requirement evidence and the independent verifier's decision. Human-only override and force-completion controls are not model tools.
- Goal maintenance is not permission to call `create_goal` or invent a token budget; reserve `create_goal` and budget changes for explicit user requests. Never claim an update was saved without a successful tool result; report unavailable persistence and do not bypass read-only or delegated tool restrictions.