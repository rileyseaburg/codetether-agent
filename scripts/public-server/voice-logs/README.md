# Voice diagnostics on the NAS

`codetether-voice-logs.service` archives allowlisted journal metadata to
`/mnt/nas/CodeTether/logs/voice/` on `//192.168.50.133/NAS`.
Files are `bridge-YYYY-MM-DD-N.jsonl`, `kokoro-YYYY-MM-DD-N.jsonl`, and
`agent-YYYY-MM-DD-N.jsonl`, rotated at 16 MiB. Existing files are not deleted.

The audio bridge returns `X-Request-ID`. Its records identify authorization,
body validation, synthesis and upstream-response stages, HTTP status, character
count and elapsed time. Kokoro records include synthesis time and audio size.
Agent records include sanitized session-route audit metadata and warning/error
categories. Raw prompts, transcripts, audio, headers, exception messages and
credentials are excluded. NAS share permissions are inherited from its mount;
this is diagnostic metadata, not a transcript archive.

NAS I/O happens in a separate service, never in the playback/request path.
The archiver acknowledges its journal cursor only after a successful NAS fsync.
An outage retries from that cursor; replayed journal IDs may be duplicated.
Journal retention bounds recovery after a prolonged outage. No claim is made
that errors occurring only on the iPhone (such as speech recognition) reach this
server. Oversized journal entries above 4 MiB are discarded.

Tests: `node --test scripts/public-server/audio/*.test.mjs scripts/public-server/voice-logs/*.test.mjs`
Deploy: `bash scripts/public-server/voice-logs/deploy.sh artifacts/<unique-run>`
Inspect: `journalctl --user -u codetether-voice-logs.service -n 20`
Live tail: `tail -F /mnt/nas/CodeTether/logs/voice/bridge-*.jsonl`