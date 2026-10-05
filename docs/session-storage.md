# Indexed session storage

Sessions use `sessions/sessions.sqlite3` in the canonical CodeTether data directory.
Managed worktrees resolve to their owning workspace's store. An explicit
`CODETETHER_DATA_DIR` still takes precedence. Per-session `<id>.json` files are
small locators, not continuously rewritten transcript snapshots.

## Cost model

`Session::save()` remains a compatibility **flush** boundary. `TrackedVec` records
the earliest dirty suffix. Appending a message encodes only that new message;
arbitrary mutable-vector access conservatively dirties the loaded suffix.
Mutation identities also detect assignment from another already-clean vector.

Normal writes cost O(new/edited bytes + metadata + indexed database operations),
not O(total conversation bytes). Unchanged flushes do not open the database,
hash the transcript, write a snapshot, or enqueue consumer work. Header metadata
is still serialized and compared; it is separate from transcript and summary
payloads. Full exports and explicitly loading full history remain O(history).

`Session::resume()` and the TUI load a bounded working tail, retaining the same
UUID. Clean working-set prefixes can be evicted after persistence without deleting
their durable records. `Session::read_messages()` reads an indexed absolute range.
The TUI pager pins absolute boundaries instead of repeatedly loading deeper tails.
`context_browse` materializes only a requested turn or listing page (default 100,
maximum 512; `offset` and `limit` select a page). Session listing reads small headers.

## Durability and conflicts

- SQLite WAL, `synchronous=FULL`, foreign keys, and short write transactions.
- Each session has an optimistic revision. A stale writer receives
  `SESSION_REVISION_CONFLICT`; it must reload rather than silently replace newer data.
- Deterministic commit nonces recognize retries after a lost acknowledgement.
- Current records and payload-bearing delta events commit atomically. Events retain
  superseded message bodies; they are not the old audit-only save markers.
- Large record bodies are stored once as immutable SHA-256-addressed blobs.
  Records and historical events reference those blobs. Reads verify their hashes.
- Tool calls remain `unknown` until an actual result is committed. The runtime
  never automatically repeats a potentially completed side effect after a crash.
- Tool dispatch propagates persistence failures before execution and after result
  recording, instead of silently advancing past failed writes.

The database, its WAL, and its blobs are one backup unit. Do not copy only a live
main database file: use SQLite's backup facilities or stop writers and checkpoint
before taking a filesystem backup. There is no automatic history garbage collection.

## Independent consumers

Recall queues session IDs, not cloned sessions. It reads unacknowledged ranges and
commits its materialized projection with its cursor. Proactive RLM summarizes closed
16-message chunks with an eight-message active tail. Summaries use absolute ranges
and are mapped into the current working window when read.

Optional S3 history archival writes immutable ranged JSONL chunks under
`<session-id>/chunks/`. Successful uploads publish chunk pointers and advance the
archive cursor; failures leave the cursor retryable. This is a new automatic
archive layout, not a periodically overwritten `<id>.jsonl` object. The explicit
legacy full-history upload/pointer APIs remain available for requested exports.
Archive consumers must use chunk pointers, not assume the legacy object exists.

Edits invalidate overlapping projections and fence in-flight acknowledgements with
consumer generations. A failed or interrupted consumer cannot mark unfinished work
processed. The local database remains authoritative; S3 is an optional archive.

## Migration and rollout

**Stop older CodeTether writers before upgrading a workspace.** An old binary cannot
understand a locator. It may still have an old snapshot in memory; no new filesystem
format can prevent an uncooperative old process from writing that snapshot again.

The first read imports a legacy snapshot, preserves an exact `<id>.legacy.json`
backup, checks the source and backup hashes, commits the import, and only then
publishes the locator. Interrupted imports can retry. A stale legacy writer is
rejected rather than allowed to replace database history. Backups and task,
verdict, and audit sidecars are retained; task logs follow the canonical locator.

Use `Session::export_json()` for a full legacy-compatible export. Do not treat
locators as exports. Do not restore only a `.legacy.json` over a migrated locator:
that is detected as a stale writer, not interpreted as a rollback.