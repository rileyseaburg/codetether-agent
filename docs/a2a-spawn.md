# `codetether spawn` — A2A Peer Runtime

Spawn an autonomous A2A agent runtime that:

- stands up its **own A2A JSON-RPC API** on a port,
- publishes an **agent card** at `/.well-known/agent.json`,
- **discovers peers** on a polling loop and registers them in the local bus,
- optionally **auto-introduces** itself to newly discovered peers,
- runs an LLM session per inbound `message/send` to actually *answer*.

This is the mode to use when you want **two (or more) terminals running two agents in different repos that can talk to each other directly**, with no central broker.

---

## TL;DR — two terminals, two repos

**Terminal 1** (Alice, repo A):
```bash
cd /path/to/repo-A
codetether spawn \
  --name alice \
  --port 4097 \
  --peer http://127.0.0.1:4098
```

**Terminal 2** (Bob, repo B):
```bash
cd /path/to/repo-B
codetether spawn \
  --name bob \
  --port 4098 \
  --peer http://127.0.0.1:4097
```

Within one discovery interval (default 15 s, override with `--discovery-interval-secs`) each side logs:

```
INFO codetether_agent::a2a::spawn: Discovered A2A peer  agent=alice  peer_name=bob  peer_url=http://127.0.0.1:4098
INFO codetether_agent::a2a::spawn: Auto-intro message sent  peer=http://127.0.0.1:4098
```

After that, either side (or any third-party tool) can drive the other over plain HTTP JSON-RPC.

---

## When to use spawn vs the other A2A modes

| Mode | Command | Bind | What it does |
|---|---|---|---|
| **Spawn** | `codetether spawn` | `127.0.0.1:4097` (configurable) | Standalone headless A2A peer with discovery + auto-intro. **Use for two-terminal / multi-repo / decentralized agent meshes when you don't need a TUI.** |
| **TUI + A2A** | `codetether tui --a2a-port <P> --a2a-peer <URL>` | configurable | Interactive TUI **plus** the same A2A peer endpoint inside the same process. **Use when you want to drive the agent yourself in the TUI and have it reachable as a peer.** Inbound A2A messages are answered by a fresh background session — they do not appear in your TUI conversation. |
| Serve | `codetether serve` | `127.0.0.1:4096` | Headless A2A API with optional mDNS. No outbound peer discovery. |
| Worker | `codetether worker --server URL` | (outbound) | Connects *to* a CodeTether server as a worker. No inbound API. |
| Swarm | `codetether swarm` | (in-process) | Spawns sub-agents inside one process. No external API. |

If you want decentralized agents that find each other and chat, `spawn` (headless) or `tui --a2a-port` (interactive) are the right modes.

---

## TUI + A2A: two interactive terminals that can message each other

If you want each side to be a TUI you drive yourself, but still reachable as an A2A peer:

**Terminal 1** (TUI in repo A, peer is repo B's TUI on :4098):
```bash
cd /path/to/repo-A
codetether tui \
  --a2a-port 4097 \
  --a2a-name alice \
  --a2a-peer http://127.0.0.1:4098
```

**Terminal 2** (TUI in repo B, peer is repo A's TUI on :4097):
```bash
cd /path/to/repo-B
codetether tui \
  --a2a-port 4098 \
  --a2a-name bob \
  --a2a-peer http://127.0.0.1:4097
```

Each TUI starts as normal and *also* exposes `/.well-known/agent.json` and the JSON-RPC endpoint on its `--a2a-port`. From inside one TUI, you can use the `http` tool (or any normal session tool) to POST a `message/send` to the other side's port. From a third terminal you can curl either side.

**Important — inbound A2A and the TUI session do not share state.** When the peer (or curl) sends `message/send`, the request is handled by a fresh `Session` spun up by the A2A handler — exactly the same way `codetether spawn` answers. The reply goes back over A2A. **The exchange does not appear in your TUI's chat view, and your TUI session does not see it.** If you want the inbound message to show up in the TUI conversation, that's the routed-into-TUI variant, which is a separate feature (not in this build).

### TUI A2A flag reference

| Flag | Default | Notes |
|---|---|---|
| `--a2a-port <PORT>` | (off) | When set, the TUI process binds an A2A endpoint on this port. Without this flag the TUI is purely interactive (no A2A surface). |
| `--a2a-hostname <HOST>` | `127.0.0.1` | Use `0.0.0.0` for off-box. Pair with `--a2a-public-url`. |
| `--a2a-public-url <URL>` | `http://<hostname>:<port>` | URL published in the agent card. |
| `--a2a-name <NAME>` | `tui-agent-<pid>` | Card name (what peers see). |
| `--a2a-description <TEXT>` | (default) | Card description. |
| `--a2a-peer <URL>` (repeatable, comma-separable) | `[]` | Peer seed URLs. Also reads `CODETETHER_A2A_PEERS`. |
| `--a2a-discovery-interval-secs <N>` | `15` | Clamped to ≥ 5. |
| `--a2a-no-auto-introduce` | (intro on) | Disable the auto-intro `message/send` to newly discovered peers. |

These flags mirror the `codetether spawn` flags one-for-one (with an `a2a-` prefix to keep them out of the TUI's own option namespace).

---

## CLI reference

```
codetether spawn [OPTIONS] [-- <PROJECT>]
```

| Flag | Default | Env | Purpose |
|---|---|---|---|
| `-n`, `--name <NAME>` | `spawned-agent-<pid>` | — | Agent name (becomes `card.name` and bus registration id). |
| `--hostname <HOST>` | `127.0.0.1` | — | Bind address. Set to `0.0.0.0` to accept off-box traffic. |
| `-p`, `--port <PORT>` | `4097` | — | Bind port. Pick a unique port per agent. |
| `--public-url <URL>` | `http://<hostname>:<port>` | — | URL published in the agent card. Set explicitly when binding `0.0.0.0` so peers know where to call back. |
| `-d`, `--description <TEXT>` | (default text) | — | Custom card description. |
| `--peer <URL>` (repeatable, comma-separable) | `[]` | `CODETETHER_A2A_PEERS` | Peer seed URLs to probe for agent cards. |
| `--discovery-interval-secs <N>` | `15` | — | How often to re-probe seeds. Minimum effective value is 5 s (clamped). |
| `--no-auto-introduce` | (intro on by default) | — | Suppress the auto-intro `message/send` sent to newly discovered peers. |
| `[PROJECT]` | cwd | — | Project directory the spawned agent will operate in. |
| `--print-logs` | off | — | Mirror tracing output to stderr (otherwise honors logfile config). |
| `--log-level DEBUG\|INFO\|WARN\|ERROR` | `INFO` | — | Tracing level. |

---

## Lifecycle (what happens when you run `spawn`)

1. **Resolve identity**: agent name + bind address + public URL. Public URL is normalized (scheme injected, trailing slash stripped).
2. **Build the agent card** (`A2AServer::default_card`) — name, description, version (from `CARGO_PKG_VERSION`), `protocolVersion: 0.3.0`, default skills (`code`, `debug`, `explain`).
3. **Initialize the bus** (`AgentBus::new`).
4. **Auto-start the S3 training sink** (best-effort; needs Vault `chat-sync-minio` creds — silent if unavailable).
5. **Register self in the bus registry** under the agent name.
6. **Announce ready** with the card's skill ids as capabilities.
7. **Resolve peer seeds** from `--peer` and `CODETETHER_A2A_PEERS`, dedup, and skip self.
8. **Start the discovery loop** (`tokio::spawn`) — see below.
9. **Bind the Axum router** (`A2AServer::router`) on `<hostname>:<port>` and serve until SIGINT.
10. On shutdown: abort discovery loop, log clean exit.

---

## HTTP API exposed by the spawned agent

The router (`src/a2a/server.rs::A2AServer::router`) mounts:

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/.well-known/agent.json` | Agent card (canonical path) |
| `GET` | `/.well-known/agent-card.json` | Agent card (compatibility alias) |
| `POST` | `/` | A2A JSON-RPC 2.0 endpoint |

### Agent card

```bash
curl -s http://127.0.0.1:4097/.well-known/agent.json | jq .
```

Returns an `AgentCard` (see `src/a2a/types.rs`):
- `name`, `description`, `url`, `version`, `protocolVersion`
- `capabilities`: `streaming: true`, `pushNotifications: false`, `stateTransitionHistory: true`
- `skills[]`: each with `id`, `name`, `description`, `tags`, `examples`, `inputModes`, `outputModes`
- `defaultInputModes` / `defaultOutputModes`
- `provider` (`organization: "CodeTether"`)
- `securitySchemes`, `security`, `signatures` (default empty)

### JSON-RPC methods

All requests are `POST /` with `Content-Type: application/json`. Wire format is JSON-RPC 2.0.

| Method | Params type | Returns |
|---|---|---|
| `message/send` | `MessageSendParams` | `Task` (or `Message`) |
| `message/stream` | `MessageSendParams` | `Task` in `working` state — poll `tasks/get` for completion |
| `tasks/get` | `TaskQueryParams` (`id`, optional `historyLength`) | `Task` |
| `tasks/cancel` | `TaskQueryParams` (`id`) | `Task` (state → `cancelled`) |

#### `message/send`

`blocking: true` (default) runs the LLM session synchronously and returns a `Task` already in `completed` (or `failed`) state with the agent's response in `status.message` and as an `artifact`.

`blocking: false` returns immediately with the task in `working`; the LLM session runs on a background tokio task, and the caller polls `tasks/get`.

```bash
curl -s -X POST http://127.0.0.1:4098/ \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "message/send",
    "params": {
      "message": {
        "messageId": "msg-001",
        "role": "user",
        "parts": [{"kind": "text", "text": "Bob, refactor src/foo.rs and report back"}]
      },
      "configuration": {
        "acceptedOutputModes": ["text/plain"],
        "blocking": false
      }
    }
  }'
```

Response (truncated):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "id": "<uuid>",
    "status": {
      "state": "working",
      "message": { ... echo of the inbound message ... },
      "timestamp": "2026-04-28T15:43:55Z"
    },
    "history": [ ... ],
    "artifacts": []
  }
}
```

#### `tasks/get`

```bash
curl -s -X POST http://127.0.0.1:4098/ \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 2,
    "method": "tasks/get",
    "params": {"id": "<task-id-from-message-send>"}
  }'
```

Returns the full `Task` including `status.state`, the agent reply in `status.message`, and any `artifacts[]`.

#### `tasks/cancel`

Refused (`-32002 TASK_NOT_CANCELABLE`) if the task is already in a terminal state.

#### Error codes (`src/a2a/types.rs`)

| Code | Constant | Meaning |
|---|---|---|
| `-32700` | `PARSE_ERROR` | JSON parse error |
| `-32600` | `INVALID_REQUEST` | Malformed JSON-RPC envelope |
| `-32601` | `METHOD_NOT_FOUND` | Unknown method |
| `-32602` | `INVALID_PARAMS` | Invalid params (e.g., empty text content) |
| `-32603` | `INTERNAL_ERROR` | Session creation / serialization failure |
| `-32001` | `TASK_NOT_FOUND` | Task id not in this server's store |
| `-32002` | `TASK_NOT_CANCELABLE` | Task already terminal |
| `-32004` | `UNSUPPORTED_OPERATION` | Method not implemented at this endpoint |

> The task store is **in-memory per server process** (`DashMap<String, Task>`). Restarting the spawned agent loses task history. Persistence belongs to the bus S3 sink (training records) or external storage.

---

## Peer discovery

Implementation: `src/a2a/spawn.rs::discovery_loop`.

Every `discovery_interval_secs` (≥ 5 s):

1. For each peer seed URL, build candidate endpoints:
   - If seed ends in `/a2a`: try as-is.
   - Otherwise: try seed, then `seed/a2a`.
2. `GET <candidate>/.well-known/agent.json` via `A2AClient` (with `CODETETHER_AUTH_TOKEN` if set).
3. First success wins. Card is registered in `bus.registry`.
4. If this is the **first** time we've seen this `endpoint::card.name` pair:
   - Log `Discovered A2A peer`.
   - If `auto_introduce` is on, send a non-blocking `message/send` with text `"Hello from <name> (<self-url>). I am online and available for A2A collaboration."`.

Discovery is **idempotent**: re-seeing a known peer re-registers the card (so cards can be refreshed) but skips the intro message.

> Discovery is **outbound-only**. To make agents truly find each other, every peer must seed at least one other peer's URL — symmetrically is fastest, but as long as the graph is connected discovery propagates over time.

### Self-skip

Peer URLs that match the agent's own `public_url` (after normalization) are dropped. This means it's safe to seed a shared peer-list env var on every node:

```bash
export CODETETHER_A2A_PEERS=http://node1:4097,http://node2:4097,http://node3:4097
```

Each node will only dial peers other than itself.

---

## Bus registry & training sink

`AgentBus` (`src/bus/mod.rs`) is the in-process pub/sub the spawned agent uses for local bookkeeping.

- `bus.registry.register(card)` — store/refresh an agent card by name.
- `handle.announce_ready(capabilities)` — mark this agent as ready for work with the listed skill ids.
- `bus::s3_sink::spawn_bus_s3_sink(bus)` — best-effort: if Vault is reachable and the `chat-sync-minio` provider is configured, the bus emits training-record JSONL batches to MinIO/S3 (`bucket: codetether-training`, `prefix: training/`, batched 100 events / 30 s).

The bus is **per-process**. Cross-process coordination is via the A2A HTTP API, not via the bus.

---

## Authentication

The spawned agent's HTTP server itself is **unauthenticated by default** — it accepts any JSON-RPC request on the bound interface. Restrict access with:

- `--hostname 127.0.0.1` (default) — loopback only.
- A reverse proxy / firewall when binding `0.0.0.0`.

The **outbound A2A client** (used by discovery and by `A2AClient`) attaches a bearer token if `CODETETHER_AUTH_TOKEN` is set in the environment. Set this on both ends if you front the spawned agent with an auth-checking proxy.

---

## Model resolution for inbound messages

When a spawned agent receives a `message/send`, it creates a fresh `Session` (`src/session/mod.rs`) and runs `session.prompt(text)`. The model used is resolved by `configure_a2a_session`:

1. `CODETETHER_DEFAULT_MODEL` env var (trimmed, non-empty).
2. Otherwise `default_model` from `Config::load()`.
3. Otherwise the session's own default (currently `glm-5.1` via `zai` provider in the default deployment).

At least one provider must be configured (Vault or env) or the session creation fails and the inbound `message/send` returns a `failed` task.

---

## Environment variables

| Var | Used by | Effect |
|---|---|---|
| `CODETETHER_A2A_PEERS` | spawn | Comma-separated peer seed URLs (alternative to `--peer`). |
| `CODETETHER_AUTH_TOKEN` | A2A client (discovery + intro) | Bearer token attached to outbound A2A calls. |
| `CODETETHER_DEFAULT_MODEL` | spawn-served sessions | Override default model used to answer inbound messages. |
| `CODETETHER_SERVER` | worker mode | Not used by `spawn`. |

---

## Cross-host setup

When the two agents are on different machines:

```bash
# Node A (10.0.0.10)
codetether spawn \
  --name alice \
  --hostname 0.0.0.0 \
  --port 4097 \
  --public-url http://10.0.0.10:4097 \
  --peer http://10.0.0.11:4097

# Node B (10.0.0.11)
codetether spawn \
  --name bob \
  --hostname 0.0.0.0 \
  --port 4097 \
  --public-url http://10.0.0.11:4097 \
  --peer http://10.0.0.10:4097
```

The `--public-url` flag matters: it's what each side publishes in its agent card and what the *other* side stores as a callback URL. Without it the card advertises `http://0.0.0.0:4097`, which is unroutable.

---

## curl recipes

```bash
# 1. Read peer's card
curl -s http://127.0.0.1:4098/.well-known/agent.json | jq .

# 2. Fire-and-forget message (non-blocking)
TASK=$(curl -s -X POST http://127.0.0.1:4098/ \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"message/send","params":{
    "message":{"messageId":"m1","role":"user",
      "parts":[{"kind":"text","text":"Summarize README.md in 3 bullets"}]},
    "configuration":{"acceptedOutputModes":["text/plain"],"blocking":false}}}' \
  | jq -r '.result.id')
echo "task_id=$TASK"

# 3. Poll for completion
curl -s -X POST http://127.0.0.1:4098/ \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tasks/get\",\"params\":{\"id\":\"$TASK\"}}" \
  | jq '.result.status.state, .result.status.message.parts'

# 4. Cancel an in-flight task
curl -s -X POST http://127.0.0.1:4098/ \
  -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tasks/cancel\",\"params\":{\"id\":\"$TASK\"}}"
```

---

## Programmatic client (Rust)

```rust
use codetether_agent::a2a::{
    client::A2AClient,
    types::{Message, MessageRole, MessageSendConfiguration, MessageSendParams, Part},
};
use std::collections::HashMap;
use uuid::Uuid;

let client = A2AClient::new("http://127.0.0.1:4098");

let card = client.get_agent_card().await?;
println!("Talking to {} ({})", card.name, card.url);

let resp = client.send_message(MessageSendParams {
    message: Message {
        message_id: Uuid::new_v4().to_string(),
        role: MessageRole::User,
        parts: vec![Part::Text { text: "ping".into() }],
        context_id: None,
        task_id: None,
        metadata: HashMap::new(),
        extensions: vec![],
    },
    configuration: Some(MessageSendConfiguration {
        accepted_output_modes: vec!["text/plain".into()],
        blocking: Some(true),
        history_length: Some(0),
        push_notification_config: None,
    }),
}).await?;
```

`A2AClient` lives in `src/a2a/client.rs` and supports `with_token(...)` for bearer auth.

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| `Failed to bind A2A peer on …: Address already in use` | Port in use (e.g., previous `spawn` still running) | Pick another `--port` or stop the prior process. |
| Discovery never logs `Discovered A2A peer` | Peer not yet listening / wrong URL / firewall | Verify with `curl http://<peer>/.well-known/agent.json`. Lower `--discovery-interval-secs 5` for faster feedback. |
| Inbound `message/send` returns `Failed to create session` | No providers configured | Check `Available providers: [...]` in the spawn log. Configure a provider via Vault or env. |
| `message/send` returns `INVALID_PARAMS: No text content in message` | All parts were `file` / `data`, no `text` part | Include at least one `{"kind":"text","text":"..."}` part. |
| Card advertises `http://0.0.0.0:4097` | Bound to `0.0.0.0` without `--public-url` | Pass `--public-url http://<reachable-host>:<port>`. |
| Auto-intro message sent but no reply | The remote agent processed the intro silently — there is no auto-reply behavior. | Send an explicit `message/send` to elicit a response. |
| Two agents reciprocally spam intro logs | Discovery interval too aggressive | Raise `--discovery-interval-secs`. Intro is only sent on **first** discovery per `endpoint::name` pair, so this should not happen unless the peer card name keeps changing. |

Enable verbose logs with `--log-level DEBUG --print-logs`. Peer probe failures are logged at `DEBUG`, so debug-level logging is the way to see *why* discovery is silent.

---

## Source map

| File | Role |
|---|---|
| `src/cli/mod.rs` (`SpawnArgs`, `Command::Spawn`) | CLI surface |
| `src/a2a/spawn.rs` | Entry point, lifecycle, discovery loop, intro sender, **`SpawnOptions` + `start_a2a_in_background`** (used by the TUI) |
| `src/tui/app/run.rs` | TUI entry point. When `--a2a-port` is set, calls `start_a2a_in_background` with the TUI's bus before entering the event loop. |
| `src/a2a/server.rs` | Axum router, JSON-RPC dispatch, message/task handlers, `default_card` |
| `src/a2a/client.rs` | Outbound `A2AClient` used by discovery + intro |
| `src/a2a/types.rs` | All wire types (`AgentCard`, `Message`, `Task`, `MessageSendParams`, JSON-RPC envelopes, error codes) |
| `src/bus/mod.rs`, `src/bus/registry.rs` | In-process bus + agent registry |
| `src/bus/s3_sink.rs` | Best-effort training-record export |
| `specification/json/a2a.json` (in `../A2A-Server-MCP/specification/json/`) | Upstream A2A protocol schema this implementation tracks |

---

## Quick mental model

> **`codetether spawn` = "be an A2A agent with my own API and find my friends."**
>
> Each spawn process is a self-contained A2A node. Nodes know about each other via seed URLs and `/.well-known/agent.json`. Communication is plain HTTP JSON-RPC 2.0 to the bound port. There is no central server, no required broker, no message bus across the wire — just agents calling each other's HTTP endpoints.
