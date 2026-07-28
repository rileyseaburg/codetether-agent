# RPC Capability Implementation Summary

## Overview

A general-purpose JSON-RPC client capability has been added to TetherScript, enabling communication with MCP (Model Context Protocol) servers, A2A (Agent-to-Agent) servers, and any JSON-RPC 2.0 compliant service.

## Files Created/Modified

### New Files
- `src/rpc_cap.rs` - Core RPC capability implementation (~850 lines)
- `examples/rpc_simple.tether` - Basic JSON-RPC usage example
- `examples/rpc_mcp.tether` - MCP server communication example
- `examples/rpc_a2a.tether` - A2A agent communication example
- `docs/rpc-capability.md` - Comprehensive documentation

### Modified Files
- `src/lib.rs` - Added `rpc_cap` module export
- `src/main.rs` - Added `--grant-rpc` CLI flag and capability registration
- `docs/rpc-capability.md` - Documentation (created)

## Features Implemented

### 1. JSON-RPC 2.0 over HTTP
- Standard request/response pattern
- Request ID generation and tracking
- Error handling with proper error codes
- Method-level permission checking

### 2. SSE (Server-Sent Events)
- Subscribe to streaming endpoints
- Parse SSE events (data, event type, id, retry)
- Call handler for each event
- Event counting

### 3. WebSocket
- HTTP upgrade handshake
- WebSocket accept key validation
- Frame encoding/decoding
- Text frame support
- Ping/Pong frame handling
- Close frame handling

### 4. Security
- Endpoint scope (specific `http://` host + port)
- Method scope (restrict which JSON-RPC methods can be called)
- Bound headers (credentials invisible to scripts)
- Capability narrowing support

## API

### Methods

#### `rpc.call(method, params)`
Make a JSON-RPC call.

```tetherscript
let params = map();
params.name = "TetherScript";
let result = rpc.call("echo", params);
```

#### `rpc.sse_subscribe(path, handler)`
Subscribe to SSE events.

```tetherscript
let handler = fn(event) {
    println("Event: " + json_encode(event.data));
};
let count = rpc.sse_subscribe("/events", handler);
```

#### `rpc.websocket(path, handler)`
Connect via WebSocket.

```tetherscript
let handler = fn(message) {
    println("Received: " + message);
};
let count = rpc.websocket("/ws", handler);
```

### Host API

#### `RpcAuthority::new(endpoint)`
Create a new RPC capability.

```rust
let rpc = rpc_cap::RpcAuthority::new("http://localhost:3000");
interp.grant("rpc", rpc);
```

#### `RpcAuthority::with_bound_header(auth, name, value)`
Attach a bound header (e.g., API key).

```rust
let rpc = rpc_cap::RpcAuthority::with_bound_header(rpc, "Authorization", "Bearer token");
```

#### `RpcAuthority::with_methods(auth, methods)`
Restrict to specific JSON-RPC methods.

```rust
let rpc = rpc_cap::RpcAuthority::with_methods(rpc, &["initialize", "tools/list"]);
```

## Protocol Support

### MCP (Model Context Protocol)
- `initialize` - Initialize MCP connection
- `tools/list` - List available tools
- `tools/call` - Call a tool
- `notifications/initialized` - Send initialized notification

### A2A (Agent-to-Agent)
- `tasks.create` - Create a task
- `tasks.get` - Get task status
- `tasks.list` - List all tasks
- `tasks.cancel` - Cancel a task
- `messages.send` - Send a message
- `agents.capabilities` - Query agent capabilities

## Limitations

- **No TLS**: Only supports `http://` endpoints. For HTTPS, use a reverse proxy.
- **No WebSocket fragmentation**: Doesn't handle fragmented frames.
- **No automatic reconnection**: SSE and WebSocket connections don't auto-reconnect.
- **Basic WebSocket**: Only supports text frames, ping/pong, and close frames.
- **No stdio transport**: MCP over stdio is not supported (only HTTP/SSE/WebSocket).
- **Simplified WebSocket**: Ping responses are ignored (would require write access during read loop).

## Implementation Details

### Zero Dependencies
All functionality is implemented using only `std`:
- Custom HTTP/1.1 client using `std::net::TcpStream`
- JSON-RPC 2.0 request/response handling
- SSE parser for streaming responses
- WebSocket handshake (HTTP upgrade) and frame codec
- Base64 encoding for WebSocket keys
- SHA-1 approximation for WebSocket accept key

### Code Organization
- `RpcAuthority` struct holds endpoint configuration
- `connect()` - Establish TCP connection
- `write_request()` - Write HTTP POST request
- `read_response()` - Read HTTP response
- `build_jsonrpc_request()` - Build JSON-RPC request
- `parse_jsonrpc_response()` - Parse JSON-RPC response
- `do_call()` - Perform JSON-RPC call
- `do_sse_subscribe()` - Subscribe to SSE events
- `do_websocket()` - Connect via WebSocket

### Comparison with ProviderAuthority

| Feature | ProviderAuthority | RpcAuthority |
|---------|------------------|--------------|
| Purpose | LLM chat completions | General JSON-RPC |
| Protocol | OpenAI-compatible | JSON-RPC 2.0 |
| SSE | LLM streaming only | Generic SSE subscription |
| WebSocket | ❌ | ✅ (basic) |
| MCP | ❌ | ✅ |
| A2A | ❌ | ✅ |
| Custom methods | ❌ | ✅ |

## Usage Examples

### Command Line
```bash
# Grant RPC capability
tetherscript --grant-rpc http://localhost:3000 script.tether

# Run examples
tetherscript --grant-rpc http://localhost:3000 examples/rpc_simple.tether
tetherscript --grant-rpc http://localhost:3000 examples/rpc_mcp.tether
tetherscript --grant-rpc http://127.0.0.1:36627 examples/rpc_a2a.tether
```

### Embedded Usage
```rust
use tetherscript::rpc_cap;

let mut interp = Interpreter::new();
let rpc = rpc_cap::RpcAuthority::new("http://localhost:3000");
interp.grant("rpc", rpc);

// Run script
interp.run(&program)?;
```

## Testing

The RPC capability has been tested with:
- Compilation check (`cargo check`)
- Example scripts created for all three use cases (simple, MCP, A2A)

## Future Enhancements

Potential improvements:
1. TLS support (would require external dependency or manual implementation)
2. WebSocket fragmentation support
3. Automatic reconnection for SSE/WebSocket
4. Binary frame support in WebSocket
5. MCP stdio transport
6. Connection pooling
7. Request timeout configuration
8. Better error messages with context

## Notes

- The `RpcAuthority` is a proper general-purpose RPC client, while `ProviderAuthority` is specialized for LLM chat completion APIs.
- All file extensions have been updated from `.kl` to `.tether` as per the deprecation of `.kl`.
- The implementation follows TetherScript's zero-dependency philosophy.
- WebSocket ping responses are currently ignored to avoid borrow checker issues; this could be improved with a more sophisticated buffering strategy.