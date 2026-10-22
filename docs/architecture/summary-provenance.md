# Summary source and policy provenance

Each new summary carries a SHA-256 digest of its complete ordered source
messages, the shared continuation-summary policy version, its producer path,
and the requested model. The digest covers serialized roles, text, attachments,
tool arguments, results, and reasoning metadata. It does not depend on absolute
message indices, so intact ranges survive working-window rebasing.

`SummaryNode::valid_for` checks the source slice and current policy.
`SummaryIndex::retain_valid` removes only unusable entries from a working index,
preserving the surviving LRU order. Loaded historical summaries without
provenance remain readable and auditable, but cannot enter a model request.
There is no destructive database migration.

Incremental derivation validates prepared entries, then merges only valid local
entries; a stale local value cannot hide a valid prepared value for the same
range. The shared projection finalizer also validates the index, covering
callers that bypass the session adapter. `context_summarize` validates before
its early cache-hit return and checks newly produced provenance before
persisting a result.

Foreground and proactive producers use the same policy instructions. Summary
inputs include the policy version, source digest, and requested model, which
also separates entries in the background RLM cache. Change
`SUMMARY_POLICY_VERSION` whenever interpretation or required summary contents
change. A cache entry produced under a previous policy must be regenerated.

The recorded model is a routing request, not proof of which backend generated
the text. The foreground producer can return deterministic compression while
background work runs; its producer tag says `background-or-fallback` for this
reason. Valid source identity is also not proof that a generated summary is
factually complete. Live model quality and provider cache-hit rates require
separate measurements.

Regression coverage includes same-size source edits, role and tool changes,
policy changes, legacy JSON, SQLite rebasing, repeated rolling-window eviction,
stale-local/valid-background conflicts, explicit tool cache hits, and model
changes in background request identity.
