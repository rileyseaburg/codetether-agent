# Session SQLite write coordination

Session records remain in one `sessions.sqlite3` file in the configured session
directory. Git worktree isolation does not create separate session databases.

Every built-in session-store mutation now goes through a dedicated writer for
the canonical database path. Session commits, summaries, projection updates,
cursor creation/acknowledgement, deletion, and schema initialization use the same
coordination boundary. Different databases have independent workers.

## Execution contract

- One persistent connection runs accepted operations in FIFO order.
- Each database admits up to 64 queued operations plus the running operation.
  Saturation returns `SESSION_WRITE_QUEUE_FULL`; it never silently drops writes.
- Async callers await a reply without occupying a Tokio blocking worker.
- Dropping a waiter does not revoke an accepted operation. Session commits keep
  their existing revision checks and durable retry identity, so a cancelled save
  can acknowledge its earlier commit without duplicating records.
- Replies follow transaction commit. Open transactions are rolled back on an
  operation error or panic; returning success with an open transaction is rejected.
- Cooperating processes use `sessions.sqlite3.writer.lock` to serialize their
  operations. A five-second lock timeout returns `SESSION_WRITER_BUSY`.
- Reads use independent WAL connections. `synchronous=FULL` remains enabled.
- Session snapshots still batch their dirty message/tool suffixes into one
  atomic commit. Unrelated operations are not combined into one transaction.

Use `coordinator::write` or `write_async` for new mutation entry points. Inside
the callback, use the supplied connection; recursively opening or submitting
through the coordinator is rejected as `SESSION_WRITE_REENTRANT`.

The registry retains workers until process exit. The advisory lock coordinates
updated CodeTether processes; arbitrary SQL clients and older binaries do not
participate in that lock and can still cause SQLite busy errors.

## Focused checks

```sh
cargo test -p codetether-session-store
cargo test -p codetether-session-store --test swarm_write_contention -- --nocapture
```
