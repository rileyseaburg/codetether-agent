# Stable request prefixes and current runtime state

The prepared foreground prompt loop and the shared context builder now use
`context_policy::request::messages::build` to assemble messages. The worker and
generic agent already used the shared context builder; the bypass was in
`context_request::attempt::build`, used by the foreground loop.

Goal accounting remains a framed runtime snapshot during composition. Request
assembly removes that frame from system instructions and sends it as explicitly
labeled user-context data. Tool schemas and static instructions retain their
identity when token usage, elapsed time, or continuation counts change.

Placement depends on the current conversation boundary:

- With a new final user message, runtime state and optional evidence precede it.
- During a continuing tool run, supplemental data follows the completed history,
  preserving the full existing message prefix and keeping call/result pairs intact.

The foreground adapter refreshes goal composition before every prepared request.
Composition removes previous frames first, adds the turn contract only once,
and inserts the current snapshot. Multiple existing frames resolve to the newest
one; an incomplete frame leaves the original prompt intact. An unreadable
journal produces an explicit runtime error snapshot. Recovering or removing the
goal does not leave stale snapshots behind. Mandatory runtime state still counts
toward the request budget and cannot silently disappear to make the request fit.

Regression tests construct consecutive requests from actual journal accounting
updates through both construction paths, then inspect Codex HTTP/WebSocket,
Anthropic, Bedrock, Google, and OpenAI chat encoding. They also cover long tool
runs, short `continue` inputs, model-switch reasoning compatibility, oversized
runtime state, and repeated composition.

These checks establish request and wire-prefix stability. They do not establish
a provider cache-hit rate, latency improvement, or cost saving; no live model
request is needed or made by the fixtures.
