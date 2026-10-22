# Durable pinned sources

Pinned context stores complete source text, its speaker, SHA-256 text hash,
and absolute transcript index. It is not limited to eight entries or a single
120-character line. Session database version three rebuilds the index from the
canonical message records, including blob-backed messages; it does not rewrite
the transcript or its event history.

`context_pin` continues to support `pin` and `unpin`. All indices are absolute,
matching `context_browse`, even when a session has a bounded working window.

```json
{"action":"pin","turn_index":1234}
```

When a later user source replaces all requirements of an earlier pinned source:

```json
{"action":"supersede","turn_index":1234,"replacement_index":1300}
```

Both page changes are committed through one ordinary session transaction.
The earlier page records `SupersededConstraint { by_turn: 1300 }`; its complete
text remains available in canonical history. Only the later source is an active
constraint. Replacement sources must be later user messages, not assistant
interpretations or tool results. A retry cannot reactivate a replacement which
has since been unpinned or superseded.

Whole-turn supersession must not be used for a partial correction to a message
containing several requirements. Use an exact source-span correction instead:

```json
{
  "action": "correct",
  "turn_index": 1234,
  "replacement_index": 1300,
  "source_text": "Use the old endpoint.",
  "source_hash": "<full SHA-256 from the source header>"
}
```

Both sources must be user-authored, and the original must be an active pin.
`source_text` must occur exactly once, including overlapping matches. The tool
records UTF-8 byte offsets, the original complete-text hash, the replacement
turn and its complete-text hash in session metadata. It pins the replacement
and saves both changes in one transaction. A failed save restores the staged
page and metadata. Multiple disjoint corrections are supported; to change an
already replaced requirement again, target its current replacement source.

Either source can be outside the loaded window. Pin, unpin, whole-turn replacement
and partial correction prepare at most two indexed canonical records; they do
not load the intervening history. Historical page changes share the normal
optimistic session revision and metadata transaction. Stale revisions or changed
source records reject the operation, and a failed save restores staged changes.
The audit log records only the selected historical rows. Tool outcomes and
message counts are unchanged, while affected context projections are invalidated.

Header rendering omits only validated replaced spans, marking each with its
source link. All other bytes remain active, and canonical originals remain
unchanged. Stale hashes, missing replacement pins, invalid UTF-8 offsets and
overlapping stored spans retain the original text and produce an explicit
`CONSTRAINT_CORRECTION_UNRESOLVED` warning. Later whole-source replacement of a
correction follows the stored forward links to its current source.

During incremental derivation, complete active sources are reserved before
optional history. Their header includes source indices, speaker roles, hashes,
and replacement links. A text-only pin covered in that header does not require
a second mandatory copy in history. Pins containing attachments still protect
the original message. Pinning assistant/tool evidence does not promote it to
user authority.

If the pinned header alone cannot fit, derivation returns an explicit budget
error instead of dropping or truncating constraints. The user can explicitly
retire obsolete requirements; the runtime does not guess that an older rule is
unimportant. Ordinary compaction and working-window eviction leave the durable
sources intact.

Conversation forks preserve pin classifications for the selected messages.
Partial and whole-source replacement links are rebased from parent absolute
indices to child indices. A prefix cut before a correction restores the earlier
instruction; a detached tail retains a correction only when both referenced
sources are included. Forking does not import source messages outside the
selected history window.

Verification covers full multiline and Unicode sources, more than eight pins,
version-two migration, repeated eviction and cold reload, absolute pin indices,
invalid and stale supersession, exact partial corrections, and request-budget reservation. See
`context-reliability-audit.md` for the completed goal's evidence and verification limits.
