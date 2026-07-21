# Mux agent task API

Mux protocol v6 gives every authenticated mux daemon a first-class agent task
lifecycle. Clients no longer need to type `codetether run` into a PTY, attach a
terminal, resize a TUI, or scrape completion markers.

Requests use the existing token-authenticated JSON-lines connection:

```json
{"type":"agent","request":{"action":"start","task_id":"task-1234","prompt":"Inspect the failing route","session_id":null,"max_steps":6}}
{"type":"agent","request":{"action":"read","task_id":"task-1234","offset":0}}
{"type":"agent","request":{"action":"cancel","task_id":"task-1234"}}
```

Responses are structured and retain the task ID:

```json
{"type":"agent","response":{"status":"accepted","task_id":"task-1234"}}
{"type":"agent","response":{"status":"output","task_id":"task-1234","data":[123,34],"next_offset":2,"running":true,"exit_code":null}}
{"type":"agent","response":{"status":"cancelled","task_id":"task-1234"}}
```

`read` is an event-driven long poll over bounded replay data. Its bytes are the
same structured JSONL run events used by `codetether run --format jsonl`, so
clients can display text deltas, tool calls, paths, provenance, and the final
response without treating terminal narration as agent speech.

The daemon permits one running agent task at a time, retains up to 128 completed
task streams with a 4 MiB replay bound per task, resumes an optional durable
session, caps agent steps, runs inside the mux workspace, and cancels the full
task process group. Protocol versions below v6 remain usable through the legacy
PTY compatibility adapter while existing mux daemons are rolled forward.
