# Mux sessions never own a workspace

A mux session may coordinate explicit files or bounded subtrees, but must never
hold an exclusive lease on the shared checkout, home directory, or filesystem.
Running a command in a directory does not confer ownership of that workspace.

## Runtime behavior

- The central tool gate removes workspace-wide claims before acquisition.
  Opaque commands, stdin, plugins, and Git operations with only a checkout-root
  scope therefore run without a workspace lease.
- Mixed batches retain their explicit file/subtree claims, not a root claim.
- Existing command approvals, sandbox boundaries, and repository restrictions
  still apply. This is a concurrency policy, not an authorization bypass.
- Unscoped subprocesses are not serialized against file edits. Prefer explicit
  edit tools or an isolated checkout when concurrent commands may touch the same files.
- The authoritative lease registry independently rejects root requests with
  `workspace_scope_forbidden`, including empty paths, dot paths, and root aliases.
  It rejects the entire acquisition: no partial claim, wait, or renewable root lease.
- File/subtree conflicts still wait using the existing bounded wait and turn lifecycle.

## Rollout

Both the mux server and its agent runtime need the updated code. Existing older
processes retain their old behavior until restarted; changing source alone does not clear live leases.