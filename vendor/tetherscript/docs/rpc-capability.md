# RPC Capability

The `RpcAuthority` capability provides general-purpose JSON-RPC client functionality for TetherScript, enabling communication with MCP (Model Context Protocol) servers, A2A (Agent-to-Agent) servers, and any JSON-RPC 2.0 compliant service.

## Features

- **JSON-RPC 2.0 over HTTP**: Standard request/response pattern
- **SSE (Server-Sent Events)**: Subscribe to streaming endpoints
- **WebSocket**: Basic bidirectional messaging (HTTP upgrade + frame codec)
- **No external dependencies**: Built with `std` only
- **Capability-based security**: Scoped to specific endpoints and methods

## Usage

### Granting the Capability

```bash
tetherscript --grant-rpc http://localhost:3000 script.tether
```

### JSON-RPC Calls

```tetherscript
// Make a JSON-RPC call
let params = map();
params.name = "TetherScript";
params.version = "0.1";

let result = rpc.call("methodName", params);

if result.is_ok() {
    let response = result.unwrap();
    println("Success: " + json_encode(response));
} else {
    println("Error: " + result.unwrap_err());
}
```

### SSE Streaming

```tetherscript
// Subscribe to SSE events
let handler = fn(event) {
    println("Event type: " + event.type);
    println("Event data: " + json_encode(event.data));
};

let count = rpc.sse_subscribe("/events", handler);
println("Received " + str(count) + " events");
```

### WebSocket Connections

```tetherscript
// Connect via WebSocket
let handler = fn(message) {
    println("Received: " + message);
};

let count = rpc.websocket("/ws", handler);
println("Received " + str(count) + " messages");
```

## Protocol Support

### MCP (Model Context Protocol)

MCP servers communicate via JSON-RPC with specific methods:

```tetherscript
// Initialize MCP connection
let params = map();
params.protocolVersion = "2024-11-05";
params.capabilities = map();
params.capabilities.tools = map();

let result = rpc.call("initialize", params);

// List available tools
let tools = rpc.call("tools/list", map());

// Call a tool
let params = map();
params.name = "tool_name";
params.arguments = map();
let result = rpc.call("tools/call", params);
```

### A2A (Agent-to-Agent)

A2A servers use JSON-RPC for task management:

```tetherscript
// Create a task
let params = map();
params.description = "Analyze codebase";
let task = rpc.call("tasks.create", params);

// Get task status
let params = map();
params.task_id = task.id;
let status = rpc.call("tasks.get", params);

// List all tasks
let tasks = rpc.call("tasks.list", map());
```

## Security

- **Endpoint scope**: Capability scoped to specific `http://` host + port
- **Method scope**: Restrict which JSON-RPC methods can be called
- **Bound headers**: Credentials attached at grant time, invisible to scripts
- **No TLS**: Plain HTTP only (use reverse proxy for HTTPS)

### Narrowing

```tetherscript
// Narrow to specific methods
let narrowed = rpc.narrow(map(
    "methods", list("initialize", "tools/list", "tools/call")
));

// Use narrowed capability
let result = narrowed.call("tools/list", map());
```

## Limitations

- **No TLS**: Only supports `http://` endpoints. For HTTPS, use a reverse proxy.
- **No WebSocket fragmentation**: Doesn't handle fragmented frames.
- **No automatic reconnection**: SSE and WebSocket connections don't auto-reconnect.
- **Basic WebSocket**: Only supports text frames, ping/pong, and close frames.
- **No stdio transport**: MCP over stdio is not supported (only HTTP/SSE/WebSocket).

## Examples

See the examples directory:
- `examples/rpc_simple.tether` - Basic JSON-RPC usage
- `examples/rpc_mcp.tether` - MCP server communication
- `examples/rpc_a2a.tether` - A2A agent communication

## Implementation Notes

The RPC capability is implemented in `src/rpc_cap.rs` with:
- Custom HTTP/1.1 client using `std::net::TcpStream`
- JSON-RPC 2.0 request/response handling
- SSE parser for streaming responses
- WebSocket handshake (HTTP upgrade) and frame codec
- Base64 encoding for WebSocket keys
- No external dependencies

## Comparison with ProviderAuthority

| Feature | ProviderAuthority | RpcAuthority |
|---------|------------------|--------------|
| Purpose | LLM chat completions | General JSON-RPC |
| Protocol | OpenAI-compatible | JSON-RPC 2.0 |
| SSE | LLM streaming only | Generic SSE subscription |
| WebSocket | ❌ | ✅ (basic) |
| MCP | ❌ | ✅ |
| A2A | ❌ | ✅ |
| Custom methods | ❌ | ✅ |

The `RpcAuthority` is a proper general-purpose RPC client, while `ProviderAuthority` is specialized for LLM chat completion APIs.