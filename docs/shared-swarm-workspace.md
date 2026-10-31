# Shared swarm editing

Local swarms use the parent's current workspace by default. Every worker has
editing tools, including workers whose task names or instructions say `inspect`,
`review`, or `test`. All workers see the same files and existing uncommitted edits.
Task classification does not silently make a shared worker read-only.

The `/swarm` command, local `codetether swarm` command, and `swarm_execute` tool
use this default. For example, a `swarm_execute` call can run twenty tasks with
`"concurrency_limit": 20`; all twenty workers can edit the same workspace.

No per-worker branches are created or merged in shared mode. Edits are visible
immediately and remain in the workspace if a worker fails. Read-only result
caches are bypassed because every worker can have filesystem side effects.
Auto-apply settings govern edit confirmation separately from tool availability.

Assign different files to workers where possible. Workers should coordinate
overlapping edits with `swarm_share` and preserve other workers' changes. Sharing
a workspace does not automatically resolve simultaneous edits to the same file.

For separate worktrees, use `"isolate_worktrees": true` with `swarm_execute`,
`codetether swarm --isolate-worktrees "Task"`, or `SwarmConfig.worktree_enabled = true`.
That opt-in retains task-based tool restrictions and per-worker integration.
The `needs_worktree` task hint applies only in this isolated mode.
Remote Kubernetes workers continue to use their remote workspace policy.