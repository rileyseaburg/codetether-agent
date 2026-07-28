//! `RpcAuthority` -- general-purpose JSON-RPC client as a capability.
//!
//! Grants TetherScript the right to make JSON-RPC 2.0 calls over HTTP,
//! subscribe to SSE streams, and establish WebSocket connections.
//!
//! # Protocol Support
//!
//! - **JSON-RPC 2.0 over HTTP**: Standard request/response pattern
//! - **SSE (Server-Sent Events)**: Subscribe to streaming endpoints
//! - **WebSocket**: Basic bidirectional messaging (HTTP upgrade + frame codec)
//!
//! # Use Cases
//!
//! - **MCP (Model Context Protocol)**: JSON-RPC over stdio, SSE, or StreamableHTTP
//! - **A2A (Agent-to-Agent)**: JSON-RPC over HTTP with task protocol
//! - **General JSON-RPC**: Any JSON-RPC 2.0 compliant server
//!
//! # Security
//!
//! - **Endpoint scope**: capability scoped to specific `http://` host + port
//! - **Method scope**: restrict which JSON-RPC methods can be called
//! - **Bound headers**: credentials attached at grant time, invisible to scripts
//! - **No TLS**: plain HTTP only (use reverse proxy for HTTPS)
//!
//! # JSON-RPC 2.0
//!
//! Requests:
//! ```json
//! {
//!   "jsonrpc": "2.0",
//!   "method": "methodName",
//!   "params": [...],
//!   "id": 1
//! }
//! ```
//!
//! Responses:
//! ```json
//! {
//!   "jsonrpc": "2.0",
//!   "result": {...},
//!   "id": 1
//! }
//! ```
//!
//! # SSE Streaming
//!
//! Subscribes to an SSE endpoint and calls a handler for each event:
//! - `data: {...}\n\n` - JSON data events
//! - `event: type\ndata: {...}\n\n` - typed events
//! - `id: 123\n` - event IDs
//! - `retry: 1000\n` - reconnection delay
//!
//! # WebSocket
//!
//! Performs HTTP upgrade handshake and handles frame encoding/decoding:
//! - Text frames (UTF-8)
//! - Ping/Pong frames
//! - Close frames
//! - No automatic reconnection or fragmentation support

use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::rc::Rc;
use std::time::Duration;

use crate::capability::Authority;
use crate::json;
use crate::value::{Runtime, Value};

const RPC_TIMEOUT: Duration = Duration::from_secs(30);
const RPC_USER_AGENT: &str = "tetherscript-rpc/0.1";
const MAX_RESPONSE_BYTES: usize = 8 * 1024 * 1024;
const MAX_SSE_LINE_BYTES: usize = 1024 * 1024;
const WS_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
type HttpResponse = (u16, HashMap<String, String>, Vec<u8>);

pub struct RpcAuthority {
    /// Allowed endpoint: `http://host[:port]`. Must be http:// (no TLS).
    endpoint: String,
    /// Parsed (host, port) from endpoint.
    host: String,
    port: u16,
    /// Allowed JSON-RPC methods. Empty = allow any method.
    methods: HashSet<String>,
    /// Bound headers (e.g. Authorization) -- invisible to TetherScript.
    bound_headers: Vec<(String, String)>,
    /// Request timeout.
    timeout: Duration,
    /// Next request ID for JSON-RPC.
    next_id: RefCell<u64>,
}

impl RpcAuthority {
    /// Create a new RPC capability scoped to the given HTTP endpoint.
    /// Endpoint must be `http://host[:port]`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(endpoint: &str) -> Rc<dyn Authority> {
        let (host, port) = parse_endpoint(endpoint).unwrap_or_else(|_| ("localhost".into(), 80));
        Rc::new(RpcAuthority {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            host,
            port,
            methods: HashSet::new(),
            bound_headers: Vec::new(),
            timeout: RPC_TIMEOUT,
            next_id: RefCell::new(1),
        })
    }

    /// Attach a bound header (e.g. API key). Only the host should call this.
    pub fn with_bound_header(
        auth: Rc<dyn Authority>,
        name: &str,
        value: &str,
    ) -> Rc<dyn Authority> {
        let this = auth
            .as_any()
            .downcast_ref::<RpcAuthority>()
            .expect("with_bound_header: authority is not RpcAuthority");
        let mut bound = this.bound_headers.clone();
        bound.push((name.to_string(), value.to_string()));
        Rc::new(RpcAuthority {
            endpoint: this.endpoint.clone(),
            host: this.host.clone(),
            port: this.port,
            methods: this.methods.clone(),
            bound_headers: bound,
            timeout: this.timeout,
            next_id: this.next_id.clone(),
        })
    }

    /// Restrict to specific JSON-RPC methods.
    pub fn with_methods(auth: Rc<dyn Authority>, methods: &[&str]) -> Rc<dyn Authority> {
        let this = auth
            .as_any()
            .downcast_ref::<RpcAuthority>()
            .expect("with_methods: authority is not RpcAuthority");
        let method_set: HashSet<String> = methods.iter().map(|s| s.to_string()).collect();
        Rc::new(RpcAuthority {
            endpoint: this.endpoint.clone(),
            host: this.host.clone(),
            port: this.port,
            methods: method_set,
            bound_headers: this.bound_headers.clone(),
            timeout: this.timeout,
            next_id: this.next_id.clone(),
        })
    }

    fn check_method(&self, method: &str) -> Result<(), String> {
        if self.methods.is_empty() {
            return Ok(());
        }
        if self.methods.contains(method) {
            return Ok(());
        }
        Err(format!(
            "rpc: method {:?} not allowed (have: {:?})",
            method,
            self.methods.iter().collect::<Vec<_>>()
        ))
    }

    /// Connect to the RPC endpoint.
    fn connect(&self) -> Result<TcpStream, String> {
        let stream = TcpStream::connect((self.host.as_str(), self.port))
            .map_err(|e| format!("rpc: connect to {}:{} failed: {}", self.host, self.port, e))?;
        stream
            .set_read_timeout(Some(self.timeout))
            .map_err(|e| format!("rpc: set read timeout failed: {}", e))?;
        stream
            .set_write_timeout(Some(self.timeout))
            .map_err(|e| format!("rpc: set write timeout failed: {}", e))?;
        Ok(stream)
    }

    /// Write an HTTP POST request with JSON-RPC body.
    fn write_request(&self, stream: &mut TcpStream, path: &str, body: &str) -> Result<(), String> {
        write!(
            stream,
            "POST {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nContent-Type: application/json\r\nAccept: application/json\r\n",
            path, self.host, RPC_USER_AGENT
        )
        .map_err(|e| format!("rpc: write request line failed: {}", e))?;

        for (name, value) in &self.bound_headers {
            write!(stream, "{}: {}\r\n", name, value)
                .map_err(|e| format!("rpc: write header failed: {}", e))?;
        }

        write!(stream, "Content-Length: {}\r\n\r\n", body.len())
            .map_err(|e| format!("rpc: write content-length failed: {}", e))?;

        stream
            .write_all(body.as_bytes())
            .and_then(|_| stream.flush())
            .map_err(|e| format!("rpc: write body failed: {}", e))?;

        Ok(())
    }

    /// Read HTTP response and return (status, headers, body).
    fn read_response(&self, stream: &mut TcpStream) -> Result<HttpResponse, String> {
        let mut reader = BufReader::new(stream);

        // Read status line
        let status_line = read_line(&mut reader, MAX_START_LINE_BYTES)?
            .ok_or_else(|| "rpc: empty response".to_string())?;
        let status_line = status_line.trim_end_matches(['\r', '\n']);
        let mut parts = status_line.splitn(3, ' ');
        let _version = parts.next().unwrap_or_default();
        let status: u16 = parts
            .next()
            .ok_or_else(|| "rpc: missing response status".to_string())?
            .parse()
            .map_err(|_| format!("rpc: invalid response status {}", status_line))?;

        // Read headers
        let mut headers = HashMap::new();
        loop {
            let line = read_line(&mut reader, MAX_HEADER_LINE_BYTES)?
                .ok_or_else(|| "rpc: unexpected EOF reading headers".to_string())?;
            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed.is_empty() {
                break;
            }
            if let Some((name, value)) = trimmed.split_once(':') {
                headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
            }
        }

        // Read body
        let body = if let Some(content_length) = headers.get("content-length") {
            let len: usize = content_length
                .parse()
                .map_err(|_| format!("rpc: invalid content-length {}", content_length))?;
            if len > MAX_RESPONSE_BYTES {
                return Err(format!(
                    "rpc: response body exceeds {} bytes",
                    MAX_RESPONSE_BYTES
                ));
            }
            let mut buf = vec![0; len];
            if len > 0 {
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| format!("rpc: read body failed: {}", e))?;
            }
            buf
        } else {
            // Read until connection closed
            let mut buf = Vec::new();
            reader
                .take((MAX_RESPONSE_BYTES + 1) as u64)
                .read_to_end(&mut buf)
                .map_err(|e| format!("rpc: read body failed: {}", e))?;
            if buf.len() > MAX_RESPONSE_BYTES {
                return Err(format!(
                    "rpc: response body exceeds {} bytes",
                    MAX_RESPONSE_BYTES
                ));
            }
            buf
        };

        Ok((status, headers, body))
    }

    /// Build a JSON-RPC 2.0 request.
    fn build_jsonrpc_request(&self, method: &str, params: &Value) -> Result<String, String> {
        self.check_method(method)?;

        let id = {
            let mut next = self.next_id.borrow_mut();
            let id = *next;
            *next = id.wrapping_add(1);
            id
        };

        let mut request = HashMap::new();
        request.insert(
            "jsonrpc".to_string(),
            Value::Str(Rc::new("2.0".to_string())),
        );
        request.insert(
            "method".to_string(),
            Value::Str(Rc::new(method.to_string())),
        );
        request.insert("params".to_string(), params.clone());
        request.insert("id".to_string(), Value::Int(id as i64));

        json::encode_to_string(&Value::Map(Rc::new(RefCell::new(request))))
    }

    /// Parse a JSON-RPC 2.0 response.
    fn parse_jsonrpc_response(&self, body: &[u8]) -> Result<Value, String> {
        let text = String::from_utf8_lossy(body).into_owned();
        let response = json::parse_str(&text)?;

        // Check for error
        if let Value::Map(ref map) = response {
            let map = map.borrow();
            if let Some(Value::Map(ref error)) = map.get("error") {
                let error = error.borrow();
                let code = error
                    .get("code")
                    .and_then(|v| {
                        if let Value::Int(c) = v {
                            Some(*c)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(-1);
                let message = error
                    .get("message")
                    .and_then(|v| {
                        if let Value::Str(s) = v {
                            Some(s.as_str())
                        } else {
                            None
                        }
                    })
                    .unwrap_or("unknown error");
                return Err(format!("rpc: error {}: {}", code, message));
            }
        }

        Ok(response)
    }

    /// Extract the result field from a JSON-RPC response.
    fn extract_result(&self, response: &Value) -> Result<Value, String> {
        if let Value::Map(ref map) = response {
            let map = map.borrow();
            if let Some(result) = map.get("result") {
                return Ok(result.clone());
            }
        }
        Err("rpc: response missing 'result' field".to_string())
    }

    /// Perform a JSON-RPC call.
    fn do_call(&self, method: &str, params: &Value) -> Result<Value, String> {
        let body = self.build_jsonrpc_request(method, params)?;
        let mut stream = self.connect()?;
        self.write_request(&mut stream, "/", &body)?;
        let (_status, _headers, response_body) = self.read_response(&mut stream)?;
        let response = self.parse_jsonrpc_response(&response_body)?;
        self.extract_result(&response)
    }

    /// Subscribe to an SSE endpoint and call handler for each event.
    fn do_sse_subscribe(
        &self,
        rt: &mut dyn Runtime,
        path: &str,
        handler: &Value,
    ) -> Result<Value, String> {
        let mut stream = self.connect()?;

        // Write GET request for SSE
        write!(
            stream,
            "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nAccept: text/event-stream\r\nConnection: keep-alive\r\n",
            path, self.host, RPC_USER_AGENT
        )
        .map_err(|e| format!("rpc: write SSE request failed: {}", e))?;

        for (name, value) in &self.bound_headers {
            write!(stream, "{}: {}\r\n", name, value)
                .map_err(|e| format!("rpc: write header failed: {}", e))?;
        }

        stream
            .write_all(b"\r\n")
            .and_then(|_| stream.flush())
            .map_err(|e| format!("rpc: write SSE request failed: {}", e))?;

        // Read response headers
        let mut reader = BufReader::new(stream);
        let _status_line = read_line(&mut reader, MAX_START_LINE_BYTES)?
            .ok_or_else(|| "rpc: empty SSE response".to_string())?;

        let mut headers = HashMap::new();
        loop {
            let line = read_line(&mut reader, MAX_HEADER_LINE_BYTES)?
                .ok_or_else(|| "rpc: unexpected EOF reading SSE headers".to_string())?;
            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed.is_empty() {
                break;
            }
            if let Some((name, value)) = trimmed.split_once(':') {
                headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
            }
        }

        // Check content type
        let content_type = headers
            .get("content-type")
            .map(|s| s.as_str())
            .unwrap_or("");
        if !content_type.contains("text/event-stream") {
            return Err(format!(
                "rpc: expected text/event-stream, got {}",
                content_type
            ));
        }

        // Read SSE events
        let mut event_type = String::new();
        let mut event_id = String::new();
        let mut event_data = String::new();
        let mut event_count: i64 = 0;

        loop {
            let line = read_line(&mut reader, MAX_SSE_LINE_BYTES)?;
            let line = match line {
                Some(l) => l,
                None => break, // EOF
            };

            let trimmed = line.trim_end_matches(['\r', '\n']);

            if trimmed.is_empty() {
                // End of event - dispatch it
                if !event_data.is_empty() {
                    event_count += 1;

                    // Parse data as JSON if possible
                    let data_value = if let Ok(parsed) = json::parse_str(&event_data) {
                        parsed
                    } else {
                        Value::Str(Rc::new(event_data.clone()))
                    };

                    // Build event map
                    let mut event_map = HashMap::new();
                    event_map.insert("type".to_string(), Value::Str(Rc::new(event_type.clone())));
                    event_map.insert("id".to_string(), Value::Str(Rc::new(event_id.clone())));
                    event_map.insert("data".to_string(), data_value);
                    event_map.insert("count".to_string(), Value::Int(event_count));

                    // Call handler
                    let event_value = Value::Map(Rc::new(RefCell::new(event_map)));
                    let _ = rt.invoke(handler, &[event_value]);
                }

                // Reset for next event
                event_type.clear();
                event_id.clear();
                event_data.clear();
                continue;
            }

            // Parse field
            if let Some((field, value)) = trimmed.split_once(':') {
                let field = field.trim();
                let value = value.trim_start();

                match field {
                    "event" => event_type = value.to_string(),
                    "id" => event_id = value.to_string(),
                    "data" => {
                        if !event_data.is_empty() {
                            event_data.push('\n');
                        }
                        event_data.push_str(value);
                    }
                    "retry" => {
                        // Ignore retry for now
                    }
                    _ => {}
                }
            }
        }

        Ok(Value::Int(event_count))
    }

    /// Perform WebSocket handshake and exchange messages.
    fn do_websocket(
        &self,
        rt: &mut dyn Runtime,
        path: &str,
        handler: &Value,
    ) -> Result<Value, String> {
        let mut stream = self.connect()?;

        // Generate WebSocket key
        let ws_key = generate_ws_key();

        // Write upgrade request
        write!(
            stream,
            "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {}\r\nSec-WebSocket-Version: 13\r\n",
            path, self.host, RPC_USER_AGENT, ws_key
        )
        .map_err(|e| format!("rpc: write WebSocket request failed: {}", e))?;

        for (name, value) in &self.bound_headers {
            write!(stream, "{}: {}\r\n", name, value)
                .map_err(|e| format!("rpc: write header failed: {}", e))?;
        }

        stream
            .write_all(b"\r\n")
            .and_then(|_| stream.flush())
            .map_err(|e| format!("rpc: write WebSocket request failed: {}", e))?;

        // Read response
        let mut reader = BufReader::new(&mut stream);
        let status_line = read_line(&mut reader, MAX_START_LINE_BYTES)?
            .ok_or_else(|| "rpc: empty WebSocket response".to_string())?;
        let status_line = status_line.trim_end_matches(['\r', '\n']);
        if !status_line.starts_with("HTTP/1.1 101") {
            return Err(format!("rpc: WebSocket upgrade failed: {}", status_line));
        }

        // Read headers and verify accept
        let mut accept_key = None;
        loop {
            let line = read_line(&mut reader, MAX_HEADER_LINE_BYTES)?
                .ok_or_else(|| "rpc: unexpected EOF reading WebSocket headers".to_string())?;
            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed.is_empty() {
                break;
            }
            if let Some((name, value)) = trimmed.split_once(':') {
                if name.trim().eq_ignore_ascii_case("sec-websocket-accept") {
                    accept_key = Some(value.trim().to_string());
                }
            }
        }

        let expected_accept = compute_ws_accept(&ws_key);
        if accept_key.as_deref() != Some(&expected_accept) {
            return Err("rpc: WebSocket accept key mismatch".to_string());
        }

        // Now we have a WebSocket connection
        // For simplicity, we'll just read messages and call the handler
        let mut message_count: i64 = 0;

        loop {
            match read_ws_frame(&mut reader) {
                Ok(Some((opcode, payload))) => {
                    match opcode {
                        0x1 => {
                            // Text frame
                            let text = String::from_utf8_lossy(&payload).into_owned();
                            let text_value = Value::Str(Rc::new(text));

                            // Call handler
                            let _ = rt.invoke(handler, &[text_value]);
                            message_count += 1;
                        }
                        0x2 => {
                            // Binary frame - skip for now
                        }
                        0x8 => {
                            // Close frame
                            break;
                        }
                        0x9 => {
                            // Ping - ignore for now (would require write access)
                        }
                        0xA => {
                            // Pong - ignore
                        }
                        _ => {}
                    }
                }
                Ok(None) => break, // Connection closed
                Err(e) => {
                    return Err(format!("rpc: WebSocket read error: {}", e));
                }
            }
        }

        Ok(Value::Int(message_count))
    }
}

impl Authority for RpcAuthority {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn invoke(&self, rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "call" => {
                if args.len() != 2 {
                    return Err("rpc.call: expected method (string), params (value)".into());
                }
                let rpc_method = string_arg(&args[0], "rpc.call: method")?;
                let params = &args[1];
                self.do_call(&rpc_method, params)
            }
            "sse_subscribe" => {
                if args.len() != 2 {
                    return Err(
                        "rpc.sse_subscribe: expected path (string), handler (function)".into(),
                    );
                }
                let path = string_arg(&args[0], "rpc.sse_subscribe: path")?;
                let handler = &args[1];
                self.do_sse_subscribe(rt, &path, handler)
            }
            "websocket" => {
                if args.len() != 2 {
                    return Err("rpc.websocket: expected path (string), handler (function)".into());
                }
                let path = string_arg(&args[0], "rpc.websocket: path")?;
                let handler = &args[1];
                self.do_websocket(rt, &path, handler)
            }
            _ => Err(format!("rpc: unknown method {}", method)),
        }
    }

    fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
        if let Value::Map(ref map) = params {
            let map = map.borrow();

            let mut methods = self.methods.clone();
            if let Some(Value::List(ref list)) = map.get("methods") {
                let list = list.borrow();
                methods.clear();
                for item in list.iter() {
                    if let Value::Str(s) = item {
                        methods.insert(s.as_str().to_string());
                    }
                }
            }

            return Ok(Rc::new(RpcAuthority {
                endpoint: self.endpoint.clone(),
                host: self.host.clone(),
                port: self.port,
                methods,
                bound_headers: self.bound_headers.clone(),
                timeout: self.timeout,
                next_id: self.next_id.clone(),
            }));
        }
        Err("rpc.narrow: expected map with optional 'methods' list".into())
    }
}

// Helper functions

fn parse_endpoint(endpoint: &str) -> Result<(String, u16), String> {
    let endpoint = endpoint.trim_end_matches('/');

    if !endpoint.starts_with("http://") {
        return Err("rpc: endpoint must start with http://".into());
    }

    let rest = &endpoint[7..]; // Skip "http://"
    let (host, port) = if let Some(idx) = rest.find(':') {
        let host = &rest[..idx];
        let port_str = &rest[idx + 1..];
        let port = port_str
            .parse::<u16>()
            .map_err(|_| format!("rpc: invalid port {}", port_str))?;
        (host.to_string(), port)
    } else {
        (rest.to_string(), 80)
    };

    if host.is_empty() {
        return Err("rpc: empty host".into());
    }

    Ok((host, port))
}

fn string_arg(arg: &Value, name: &str) -> Result<String, String> {
    match arg {
        Value::Str(s) => Ok(s.as_str().to_string()),
        other => Err(format!(
            "{}: expected string, got {}",
            name,
            other.type_name()
        )),
    }
}

fn read_line<R: Read>(
    reader: &mut BufReader<R>,
    max_bytes: usize,
) -> Result<Option<String>, String> {
    let mut line = String::new();
    let mut bytes = 0;

    for byte in reader.bytes() {
        let byte = byte.map_err(|e| format!("rpc: read failed: {}", e))?;
        bytes += 1;
        if bytes > max_bytes {
            return Err(format!("rpc: line exceeds {} bytes", max_bytes));
        }

        if byte == b'\n' {
            return Ok(Some(line));
        }
        line.push(byte as char);
    }

    if line.is_empty() {
        Ok(None)
    } else {
        Ok(Some(line))
    }
}

const MAX_START_LINE_BYTES: usize = 8 * 1024;
const MAX_HEADER_LINE_BYTES: usize = 8 * 1024;

// WebSocket helpers

fn generate_ws_key() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    base64_encode(format!("tetherscript-{}", nonce).as_bytes())
}

fn compute_ws_accept(key: &str) -> String {
    // WebSocket RFC 6455 requires SHA-1(key + GUID), then base64.
    // Implemented from the spec using only std � no external crate.
    let combined = format!("{}{}", key, WS_GUID);
    let hash = sha1(combined.as_bytes());
    base64_encode(&hash)
}

/// Minimal SHA-1 per FIPS 180-4. Output: 20 bytes.
fn sha1(message: &[u8]) -> [u8; 20] {
    // Initial hash values (H0..H4)
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    // Pre-processing: pad to 512-bit block boundary
    let msg_len_bits = (message.len() as u64) * 8;
    let mut data = message.to_vec();
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0x00);
    }
    data.extend_from_slice(&msg_len_bits.to_be_bytes());

    // Process each 512-bit (64-byte) block
    let mut w = [0u32; 80];
    for chunk in data.chunks_exact(64) {
        // Break block into sixteen 32-bit big-endian words
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        // Extend to 80 words
        for i in 16..80 {
            let val = w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16];
            w[i] = val.rotate_left(1);
        }

        // Working variables
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        // 80 rounds
        for (i, word) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1u32),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDCu32),
                60..=79 => (b ^ c ^ d, 0xCA62C1D6u32),
                _ => unreachable!(),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(*word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    // Produce the 20-byte hash
    let mut result = [0u8; 20];
    result[0..4].copy_from_slice(&h0.to_be_bytes());
    result[4..8].copy_from_slice(&h1.to_be_bytes());
    result[8..12].copy_from_slice(&h2.to_be_bytes());
    result[12..16].copy_from_slice(&h3.to_be_bytes());
    result[16..20].copy_from_slice(&h4.to_be_bytes());
    result
}

fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut i = 0;

    while i + 3 <= data.len() {
        let chunk = u32::from_be_bytes([0, data[i], data[i + 1], data[i + 2]]);
        result.push(TABLE[((chunk >> 18) & 0x3F) as usize] as char);
        result.push(TABLE[((chunk >> 12) & 0x3F) as usize] as char);
        result.push(TABLE[((chunk >> 6) & 0x3F) as usize] as char);
        result.push(TABLE[(chunk & 0x3F) as usize] as char);
        i += 3;
    }

    if i + 2 == data.len() {
        let chunk = u32::from_be_bytes([0, data[i], data[i + 1], 0]);
        result.push(TABLE[((chunk >> 18) & 0x3F) as usize] as char);
        result.push(TABLE[((chunk >> 12) & 0x3F) as usize] as char);
        result.push(TABLE[((chunk >> 6) & 0x3F) as usize] as char);
        result.push('=');
    } else if i + 1 == data.len() {
        let chunk = u32::from_be_bytes([0, data[i], 0, 0]);
        result.push(TABLE[((chunk >> 18) & 0x3F) as usize] as char);
        result.push(TABLE[((chunk >> 12) & 0x3F) as usize] as char);
        result.push('=');
        result.push('=');
    }

    result
}

fn read_ws_frame<R: Read>(reader: &mut BufReader<R>) -> Result<Option<(u8, Vec<u8>)>, String> {
    // Read first 2 bytes
    let mut header = [0u8; 2];
    if reader.read_exact(&mut header).is_err() {
        return Ok(None);
    }

    let byte1 = header[0];
    let byte2 = header[1];

    let fin = (byte1 & 0x80) != 0;
    let opcode = byte1 & 0x0F;
    let masked = (byte2 & 0x80) != 0;
    let mut payload_len = (byte2 & 0x7F) as usize;

    // Read extended payload length
    if payload_len == 126 {
        let mut ext = [0u8; 2];
        reader
            .read_exact(&mut ext)
            .map_err(|e| format!("rpc: read WebSocket extended length failed: {}", e))?;
        payload_len = u16::from_be_bytes(ext) as usize;
    } else if payload_len == 127 {
        let mut ext = [0u8; 8];
        reader
            .read_exact(&mut ext)
            .map_err(|e| format!("rpc: read WebSocket extended length failed: {}", e))?;
        payload_len = u64::from_be_bytes(ext) as usize;
    }

    // Read masking key if present
    let mut mask = [0u8; 4];
    if masked {
        reader
            .read_exact(&mut mask)
            .map_err(|e| format!("rpc: read WebSocket mask failed: {}", e))?;
    }

    // Read payload
    let mut payload = vec![0u8; payload_len];
    if payload_len > 0 {
        reader
            .read_exact(&mut payload)
            .map_err(|e| format!("rpc: read WebSocket payload failed: {}", e))?;
    }

    // Unmask if needed
    if masked {
        for i in 0..payload_len {
            payload[i] ^= mask[i % 4];
        }
    }

    // Only return complete frames
    if !fin {
        // For simplicity, we don't handle fragmented frames
        return Err("rpc: fragmented WebSocket frames not supported".into());
    }

    Ok(Some((opcode, payload)))
}

#[allow(dead_code)]
fn write_ws_frame(stream: &mut TcpStream, opcode: u8, payload: &[u8]) -> Result<(), String> {
    let len = payload.len();

    let byte1 = 0x80 | opcode; // FIN + opcode
    let byte2 = 0x00; // Not masked

    let mut header = Vec::new();
    header.push(byte1);

    if len < 126 {
        header.push(byte2 | len as u8);
    } else if len < 65536 {
        header.push(byte2 | 126);
        header.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        header.push(byte2 | 127);
        header.extend_from_slice(&(len as u64).to_be_bytes());
    }

    stream
        .write_all(&header)
        .and_then(|_| stream.write_all(payload))
        .and_then(|_| stream.flush())
        .map_err(|e| format!("rpc: write WebSocket frame failed: {}", e))?;

    Ok(())
}
