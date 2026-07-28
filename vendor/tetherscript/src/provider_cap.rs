//! `ProviderAuthority` — LLM provider calls as a capability.
//!
//! Grants TetherScript the right to call LLM chat completion APIs over HTTP(S).
//! Uses the OpenAI-compatible `/v1/chat/completions` endpoint shape, which is
//! also supported by Ollama, vLLM, LM Studio, Cerebras, and many other backends.
//!
//! # Protocol
//!
//! The provider speaks HTTP/1.1 over plain TCP or platform OpenSSL TLS:
//!
//! - **Ollama** (`http://localhost:11434`)
//! - **LM Studio** (`http://localhost:1234`)
//! - **Cerebras** (`https://api.cerebras.ai`) — GLM 4.7 and other models
//! - **vLLM** (behind a local reverse proxy or direct)
//! - **Any OpenAI-compatible API** (http or https)
//!
//! # GLM 4.7 / Cerebras support
//!
//! The provider supports GLM 4.7-specific parameters:
//! - `max_completion_tokens` (GLM 4.7 uses this instead of `max_tokens`)
//! - `reasoning_effort` (`"none"` to disable, omit to enable)
//! - `clear_thinking` (preserve reasoning traces across turns)
//! - `top_p` (sampling parameter)
//!
//! Streaming responses include `delta.reasoning` content for thinking models,
//! surfaced alongside `delta.content` tokens.
//!
//! # SSE streaming
//!
//! - `chat(messages)` — collects the full response and returns the assistant
//!   message content as a string.
//! - `chat_json(messages)` — collects the full response and returns the raw
//!   JSON response as a parsed TetherScript value.
//! - `stream(messages, handler)` — calls `handler(token, delta)` for each SSE
//!   chunk. The handler receives the accumulated text and the new delta.
//!
//! # Security
//!
//! - **Endpoint scope**: the capability is scoped to a specific `http://` or
//!   `https://` host + port at grant time. The script cannot call a different server.
//! - **Model scope**: optionally restrict which models the script can request.
//! - **Bound headers**: API keys are attached as bound headers at grant time
//!   and are invisible to TetherScript code.
//! - **Max tokens budget**: optional cap on total response tokens per call.
//!
//! # Narrowing
//!
//! A provider capability can be narrowed to restrict:
//! - `models`: subset of allowed models
//! - `max_tokens`: lower the per-call token cap
//! - `path_prefix`: restrict to a sub-path of the API

use std::any::Any;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::rc::Rc;
use std::time::Duration;

use crate::tls::TlsConnector;

use crate::capability::Authority;
use crate::json;
use crate::value::{Runtime, Value};

const PROVIDER_TIMEOUT: Duration = Duration::from_secs(120);
const PROVIDER_USER_AGENT: &str = "tetherscript-provider/0.2";
const MAX_RESPONSE_BYTES: usize = 8 * 1024 * 1024;
const MAX_SSE_LINE_BYTES: usize = 1024 * 1024;

/// TLS or plain HTTP scheme.
#[derive(Clone, Debug, PartialEq)]
enum Scheme {
    Http,
    Https,
}

/// Trait object for a stream that supports both Read and Write.
trait NetStream: Read + Write {}
impl NetStream for TcpStream {}
impl NetStream for crate::tls::TlsStream {}

pub struct ProviderAuthority {
    /// Allowed endpoint: `http://host[:port]` or `https://host[:port]`.
    endpoint: String,
    /// TLS or plain.
    scheme: Scheme,
    /// Parsed host from endpoint.
    host: String,
    /// Parsed port from endpoint.
    port: u16,
    /// Allowed model names. Empty = allow any model.
    models: HashSet<String>,
    /// URL path prefix for the API (default: `/v1/chat/completions`).
    path: String,
    /// Bound headers (e.g. Authorization) — invisible to TetherScript.
    bound_headers: Vec<(String, String)>,
    /// Per-call max tokens cap. 0 = no cap.
    max_tokens: u64,
    /// Request timeout.
    timeout: Duration,
}

impl ProviderAuthority {
    /// Create a new provider capability scoped to the given HTTP endpoint.
    /// Endpoint must be `http://host[:port]`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(endpoint: &str) -> Rc<dyn Authority> {
        let (scheme, host, port) =
            parse_endpoint(endpoint).unwrap_or_else(|| (Scheme::Http, "localhost".into(), 80));
        Rc::new(ProviderAuthority {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            scheme,
            host,
            port,
            models: HashSet::new(),
            path: "/v1/chat/completions".to_string(),
            bound_headers: Vec::new(),
            max_tokens: 0,
            timeout: PROVIDER_TIMEOUT,
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
            .downcast_ref::<ProviderAuthority>()
            .expect("with_bound_header: authority is not ProviderAuthority");
        let mut bound = this.bound_headers.clone();
        bound.push((name.to_string(), value.to_string()));
        Rc::new(ProviderAuthority {
            endpoint: this.endpoint.clone(),
            scheme: this.scheme.clone(),
            host: this.host.clone(),
            port: this.port,
            models: this.models.clone(),
            path: this.path.clone(),
            bound_headers: bound,
            max_tokens: this.max_tokens,
            timeout: this.timeout,
        })
    }

    /// Restrict to a specific model.
    pub fn with_model(auth: Rc<dyn Authority>, model: &str) -> Rc<dyn Authority> {
        let this = auth
            .as_any()
            .downcast_ref::<ProviderAuthority>()
            .expect("with_model: authority is not ProviderAuthority");
        let mut models = this.models.clone();
        models.insert(model.to_string());
        Rc::new(ProviderAuthority {
            endpoint: this.endpoint.clone(),
            scheme: this.scheme.clone(),
            host: this.host.clone(),
            port: this.port,
            models,
            path: this.path.clone(),
            bound_headers: this.bound_headers.clone(),
            max_tokens: this.max_tokens,
            timeout: this.timeout,
        })
    }

    /// Set the API path (default: `/v1/chat/completions`).
    pub fn with_path(auth: Rc<dyn Authority>, path: &str) -> Rc<dyn Authority> {
        let this = auth
            .as_any()
            .downcast_ref::<ProviderAuthority>()
            .expect("with_path: authority is not ProviderAuthority");
        Rc::new(ProviderAuthority {
            endpoint: this.endpoint.clone(),
            scheme: this.scheme.clone(),
            host: this.host.clone(),
            port: this.port,
            models: this.models.clone(),
            path: path.to_string(),
            bound_headers: this.bound_headers.clone(),
            max_tokens: this.max_tokens,
            timeout: this.timeout,
        })
    }

    fn check_model(&self, model: &str) -> Result<(), String> {
        if self.models.is_empty() {
            return Ok(());
        }
        if self.models.contains(model) {
            return Ok(());
        }
        Err(format!(
            "provider: model {:?} not allowed (have: {:?})",
            model,
            self.models.iter().collect::<Vec<_>>()
        ))
    }

    /// Build a chat request body in TetherScript value form, then encode to JSON.
    fn build_request_body(&self, args: &[Value]) -> Result<String, String> {
        // Expected args: messages (list of maps), optional overrides map
        let messages_val = args
            .first()
            .ok_or("provider.chat: expected messages argument")?;

        let mut body = HashMap::new();
        body.insert(
            "messages".to_string(),
            sanitize_chat_messages(messages_val.clone())?,
        );

        // Extract model from overrides or default
        let mut model = String::new();
        if let Some(Value::Map(m)) = args.get(1) {
            let m = m.borrow();
            if let Some(Value::Str(m_name)) = m.get("model") {
                model = m_name.to_string();
                body.insert("model".to_string(), Value::Str(Rc::new(model.clone())));
            }
            if let Some(Value::Int(mt)) = m.get("max_tokens") {
                let capped = if self.max_tokens > 0 {
                    (*mt).min(self.max_tokens as i64)
                } else {
                    *mt
                };
                body.insert("max_tokens".to_string(), Value::Int(capped));
            }
            if let Some(Value::Float(t)) = m.get("temperature") {
                body.insert("temperature".to_string(), Value::Float(*t));
            }
            if let Some(Value::Str(s)) = m.get("stream") {
                body.insert("stream".to_string(), Value::Str(s.clone()));
            }

            // top_p sampling (used by GLM 4.7 and others)
            if let Some(Value::Float(p)) = m.get("top_p") {
                body.insert("top_p".to_string(), Value::Float(*p));
            }

            // GLM 4.7 / Cerebras parameters
            if let Some(Value::Int(mt)) = m.get("max_completion_tokens") {
                let capped = if self.max_tokens > 0 {
                    (*mt).min(self.max_tokens as i64)
                } else {
                    *mt
                };
                body.insert("max_completion_tokens".to_string(), Value::Int(capped));
            }
            if let Some(Value::Str(effort)) = m.get("reasoning_effort") {
                body.insert("reasoning_effort".to_string(), Value::Str(effort.clone()));
            }
            if let Some(Value::Bool(ct)) = m.get("clear_thinking") {
                body.insert("clear_thinking".to_string(), Value::Bool(*ct));
            }
            // Bool-valued stream parameter
            if let Some(Value::Bool(b)) = m.get("stream") {
                body.insert("stream".to_string(), Value::Bool(*b));
            }
        }

        if !model.is_empty() {
            self.check_model(&model)?;
        }

        // If stream is not set and we want streaming, set it
        let body_val = Value::Map(Rc::new(RefCell::new(body)));
        json::encode_to_string(&body_val)
    }

    /// Send HTTP POST and collect full response (non-streaming).
    fn do_chat(&self, body: &str) -> Result<Value, String> {
        let response_bytes = self.http_post(body)?;
        let response_text = String::from_utf8_lossy(&response_bytes).into_owned();

        // Parse the JSON response
        let parsed = json::parse_str(&response_text)?;

        // Extract the assistant message content
        extract_chat_content(&parsed)
    }

    /// Send HTTP POST and return the raw JSON response as a parsed value.
    fn do_chat_json(&self, body: &str) -> Result<Value, String> {
        let response_bytes = self.http_post(body)?;
        let response_text = String::from_utf8_lossy(&response_bytes).into_owned();
        let parsed = json::parse_str(&response_text).map_err(|e| {
            format!(
                "provider: failed to parse JSON response: {}; content: {}",
                e,
                response_text.trim()
            )
        })?;
        response_error(&parsed).map_or(Ok(parsed), Err)
    }

    /// Send HTTP POST with streaming and call handler for each token.
    fn do_stream(
        &self,
        rt: &mut dyn Runtime,
        body: &str,
        handler: &Value,
    ) -> Result<Value, String> {
        // Set stream=true in body
        let body_val = json::parse_str(body)?;
        let patched_body = if let Value::Map(m) = &body_val {
            let mut map = m.borrow().clone();
            map.insert("stream".to_string(), Value::Bool(true));
            let patched = Value::Map(Rc::new(RefCell::new(map)));
            json::encode_to_string(&patched)?
        } else {
            return Err("provider.stream: request body must be a map".into());
        };

        let mut stream = self.connect()?;
        self.write_request(&mut *stream, &patched_body)?;

        let mut reader = BufReader::new(stream);
        let mut accumulated = String::new();

        loop {
            let line = read_line(&mut reader)?;
            let line = match line {
                Some(l) => l,
                None => break, // EOF
            };

            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed.is_empty() {
                continue;
            }

            // SSE lines look like: data: {...}\n
            // or: data: [DONE]\n
            let data = match trimmed.strip_prefix("data: ") {
                Some(d) => d.trim(),
                None => continue,
            };

            if data == "[DONE]" {
                break;
            }

            // Parse the SSE chunk JSON
            let chunk = json::parse_str(data)?;
            let (content_delta, reasoning_delta) = extract_stream_deltas(&chunk);

            // Surface reasoning tokens (GLM 4.7 thinking models)
            if let Some(reasoning_text) = reasoning_delta {
                accumulated.push_str(&reasoning_text);
                let args = vec![
                    Value::Str(Rc::new(accumulated.clone())),
                    Value::Str(Rc::new(reasoning_text)),
                ];
                rt.invoke(handler, &args)?;
            }

            if let Some(delta_text) = content_delta {
                accumulated.push_str(&delta_text);

                // Call the handler: handler(accumulated, delta_text)
                let args = vec![
                    Value::Str(Rc::new(accumulated.clone())),
                    Value::Str(Rc::new(delta_text)),
                ];
                rt.invoke(handler, &args)?;
            }
        }

        Ok(Value::Str(Rc::new(accumulated)))
    }

    fn http_post(&self, body: &str) -> Result<Vec<u8>, String> {
        let mut stream = self.connect()?;
        self.write_request(&mut *stream, body)?;
        self.read_response(stream)
    }

    fn connect(&self) -> Result<Box<dyn NetStream>, String> {
        match self.scheme {
            Scheme::Http => {
                let tcp = TcpStream::connect((self.host.as_str(), self.port)).map_err(|e| {
                    format!(
                        "provider: connect to {}:{} failed: {}",
                        self.host, self.port, e
                    )
                })?;
                tcp.set_read_timeout(Some(self.timeout))
                    .map_err(|e| format!("provider: set read timeout: {}", e))?;
                tcp.set_write_timeout(Some(self.timeout))
                    .map_err(|e| format!("provider: set write timeout: {}", e))?;
                Ok(Box::new(tcp))
            }
            Scheme::Https => {
                let connector = TlsConnector::new()
                    .map_err(|e| format!("provider: create TLS connector: {}", e))?;
                let tls_stream = connector.connect(&self.host, self.port).map_err(|e| {
                    format!("provider: TLS handshake with {} failed: {}", self.host, e)
                })?;
                Ok(Box::new(tls_stream))
            }
        }
    }

    fn write_request(&self, stream: &mut dyn Write, body: &str) -> Result<(), String> {
        let default_port: u16 = match self.scheme {
            Scheme::Http => 80,
            Scheme::Https => 443,
        };
        let host_header = if self.port == default_port {
            self.host.clone()
        } else {
            format!("{}:{}", self.host, self.port)
        };

        write!(
            stream,
            "POST {} HTTP/1.1\r\n\
             Host: {}\r\n\
             User-Agent: {}\r\n\
             Accept: application/json\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n",
            self.path,
            host_header,
            PROVIDER_USER_AGENT,
            body.len()
        )
        .map_err(|e| format!("provider: write request head: {}", e))?;

        for (name, value) in &self.bound_headers {
            write!(stream, "{}: {}\r\n", name, value)
                .map_err(|e| format!("provider: write bound header: {}", e))?;
        }

        stream
            .write_all(b"\r\n")
            .map_err(|e| format!("provider: write separator: {}", e))?;
        stream
            .write_all(body.as_bytes())
            .map_err(|e| format!("provider: write body: {}", e))?;
        stream
            .flush()
            .map_err(|e| format!("provider: flush: {}", e))?;

        Ok(())
    }

    fn read_response(&self, stream: Box<dyn NetStream>) -> Result<Vec<u8>, String> {
        let mut reader = BufReader::new(stream);

        // Read status line
        let status_line =
            read_line(&mut reader)?.ok_or_else(|| "provider: empty response".to_string())?;
        let status = parse_status_code(&status_line)?;
        if !(200..300).contains(&status) {
            // Read body for error info
            let mut body = Vec::new();
            let _ = reader
                .take((MAX_RESPONSE_BYTES + 1) as u64)
                .read_to_end(&mut body);
            let text = String::from_utf8_lossy(&body);
            let detail = json::parse_str(text.trim())
                .ok()
                .and_then(|v| response_error(&v))
                .unwrap_or_else(|| text.trim().to_string());
            return Err(format!(
                "provider: HTTP {} response: {}",
                status,
                detail
            ));
        }

        // Read headers
        let mut content_length: Option<usize> = None;
        let mut chunked = false;
        loop {
            let line = read_line(&mut reader)?
                .ok_or_else(|| "provider: unexpected EOF reading headers".to_string())?;
            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed.is_empty() {
                break;
            }
            if let Some((name, value)) = trimmed.split_once(':') {
                let name = name.trim().to_ascii_lowercase();
                let value = value.trim();
                match name.as_str() {
                    "content-length" => {
                        content_length = Some(value.parse().unwrap_or(0));
                    }
                    "transfer-encoding" => {
                        chunked = value
                            .split(',')
                            .any(|part| part.trim().eq_ignore_ascii_case("chunked"));
                    }
                    _ => {}
                }
            }
        }

        // Read body
        let body = if chunked {
            read_chunked(&mut reader)?
        } else if let Some(len) = content_length {
            if len > MAX_RESPONSE_BYTES {
                return Err(format!(
                    "provider: response body {} exceeds {} bytes",
                    len, MAX_RESPONSE_BYTES
                ));
            }
            let mut buf = vec![0u8; len];
            if len > 0 {
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| format!("provider: read body: {}", e))?;
            }
            buf
        } else {
            let mut buf = Vec::new();
            reader
                .take((MAX_RESPONSE_BYTES + 1) as u64)
                .read_to_end(&mut buf)
                .map_err(|e| format!("provider: read body: {}", e))?;
            if buf.len() > MAX_RESPONSE_BYTES {
                return Err(format!(
                    "provider: response body exceeds {} bytes",
                    MAX_RESPONSE_BYTES
                ));
            }
            buf
        };

        Ok(body)
    }
}

// --- I/O helpers (generic over NetStream trait object) ---

fn read_line(reader: &mut BufReader<Box<dyn NetStream>>) -> Result<Option<String>, String> {
    let mut out = Vec::new();
    loop {
        let available = reader
            .fill_buf()
            .map_err(|e| format!("provider: read: {}", e))?;
        if available.is_empty() {
            if out.is_empty() {
                return Ok(None);
            }
            break;
        }
        let take_len = available
            .iter()
            .position(|b| *b == b'\n')
            .map_or(available.len(), |pos| pos + 1);
        if out.len() + take_len > MAX_SSE_LINE_BYTES {
            return Err(format!(
                "provider: line exceeds {} bytes",
                MAX_SSE_LINE_BYTES
            ));
        }
        let has_newline = available[..take_len].last() == Some(&b'\n');
        out.extend_from_slice(&available[..take_len]);
        reader.consume(take_len);
        if has_newline {
            break;
        }
    }
    String::from_utf8(out)
        .map(Some)
        .map_err(|e| format!("provider: invalid UTF-8: {}", e))
}

fn read_chunked(reader: &mut BufReader<Box<dyn NetStream>>) -> Result<Vec<u8>, String> {
    let mut body = Vec::new();
    loop {
        let line = read_line(reader)?
            .ok_or_else(|| "provider: unexpected EOF reading chunk size".to_string())?;
        let size_text = line
            .trim_end_matches(['\r', '\n'])
            .split(';')
            .next()
            .unwrap_or_default()
            .trim();
        let size = usize::from_str_radix(size_text, 16)
            .map_err(|_| format!("provider: invalid chunk size {:?}", size_text))?;
        if size == 0 {
            break;
        }
        if body.len() + size > MAX_RESPONSE_BYTES {
            return Err(format!(
                "provider: chunked body exceeds {} bytes",
                MAX_RESPONSE_BYTES
            ));
        }
        let start = body.len();
        body.resize(start + size, 0);
        reader
            .read_exact(&mut body[start..])
            .map_err(|e| format!("provider: read chunk: {}", e))?;
        let mut crlf = [0u8; 2];
        reader
            .read_exact(&mut crlf)
            .map_err(|e| format!("provider: read chunk terminator: {}", e))?;
    }
    Ok(body)
}

fn parse_status_line(status_line: &str) -> Result<(u16, String), String> {
    let line = status_line.trim_end_matches(['\r', '\n']);
    let mut parts = line.splitn(3, ' ');
    let version = parts.next().unwrap_or_default();
    if !version.starts_with("HTTP/") {
        return Err(format!("provider: invalid status line: {:?}", line));
    }
    let status: u16 = parts
        .next()
        .ok_or_else(|| "provider: missing status code".to_string())?
        .parse()
        .map_err(|_| format!("provider: invalid status code in: {:?}", line))?;
    let reason = parts.next().unwrap_or_default().to_string();
    Ok((status, reason))
}

fn parse_status_code(status_line: &str) -> Result<u16, String> {
    let (code, _) = parse_status_line(status_line)?;
    Ok(code)
}

// --- Response extraction ---

fn sanitize_chat_messages(value: Value) -> Result<Value, String> {
    let messages = match value {
        Value::List(messages) => messages,
        other => {
            return Err(format!(
                "provider.chat_json: messages must be a list, got {}",
                other.type_name()
            ));
        }
    };

    let sanitized = messages
        .borrow()
        .iter()
        .map(sanitize_chat_message)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Value::List(Rc::new(RefCell::new(sanitized))))
}

fn sanitize_chat_message(message: &Value) -> Result<Value, String> {
    let map = match message {
        Value::Map(map) => map,
        other => {
            return Err(format!(
                "provider.chat_json: each message must be a map, got {}",
                other.type_name()
            ));
        }
    };

    let mut out = map.borrow().clone();
    let role = match out.get("role") {
        Some(Value::Str(role)) => role.as_str().to_string(),
        _ => String::new(),
    };

    let content = out.get("content").cloned().unwrap_or(Value::Nil);
    out.insert("content".to_string(), sanitize_message_content(&role, content));

    Ok(Value::Map(Rc::new(RefCell::new(out))))
}

fn sanitize_message_content(role: &str, content: Value) -> Value {
    match content {
        Value::Str(text) => {
            if role == "tool" && text.is_empty() {
                Value::Str(Rc::new(" ".to_string()))
            } else {
                Value::Str(text)
            }
        }
        Value::List(parts) => {
            let mut non_empty = Vec::new();
            for part in parts.borrow().iter() {
                match part {
                    Value::Map(part_map) => {
                        let part_out = part_map.borrow().clone();
                        if matches!(part_out.get("type"), Some(Value::Str(t)) if t.as_str() == "text")
                            && matches!(part_out.get("text"), Some(Value::Str(t)) if t.is_empty())
                        {
                            continue;
                        }
                        non_empty.push(Value::Map(Rc::new(RefCell::new(part_out))));
                    }
                    other => non_empty.push(other.clone()),
                }
            }
            if role == "tool" && non_empty.is_empty() {
                Value::Str(Rc::new(" ".to_string()))
            } else {
                Value::List(Rc::new(RefCell::new(non_empty)))
            }
        }
        Value::Nil if role == "tool" => Value::Str(Rc::new(" ".to_string())),
        other => other,
    }
}

fn response_error(response: &Value) -> Option<String> {
    let map = match response {
        Value::Map(map) => map.borrow(),
        _ => return None,
    };
    let error = map.get("error")?;
    Some(format!("provider api error: {}", error))
}

/// Extract the assistant message content from a non-streaming chat response.
fn extract_chat_content(response: &Value) -> Result<Value, String> {
    // Response shape: { choices: [{ message: { content: "..." } }] }
    let choices = match response {
        Value::Map(m) => m.borrow().get("choices").cloned(),
        _ => return Err("provider: response is not a map".into()),
    };

    let choices_list = match choices {
        Some(Value::List(l)) => l,
        Some(_) => return Err("provider: response.choices is not a list".into()),
        None => return Err("provider: response missing choices field".into()),
    };

    let first_choice = choices_list
        .borrow()
        .first()
        .cloned()
        .ok_or_else(|| "provider: response.choices is empty".to_string())?;

    let message = match &first_choice {
        Value::Map(m) => m.borrow().get("message").cloned(),
        _ => return Err("provider: choice is not a map".into()),
    };

    match message {
        Some(Value::Map(m)) => match m.borrow().get("content").cloned() {
            Some(Value::Str(content)) => Ok(Value::Str(content)),
            Some(other) => Err(format!(
                "provider: message.content is not a string, got {}",
                other.type_name()
            )),
            None => Err("provider: message missing content field".into()),
        },
        Some(_) => Err("provider: choice.message is not a map".into()),
        None => Err("provider: choice missing message field".into()),
    }
}

/// Extract both `content` and `reasoning` deltas from a streaming SSE chunk.
/// GLM 4.7 uses `delta.reasoning` for thinking tokens and `delta.content`
/// for output tokens.
fn extract_stream_deltas(chunk: &Value) -> (Option<String>, Option<String>) {
    let choices = match chunk {
        Value::Map(m) => m.borrow().get("choices").cloned(),
        _ => return (None, None),
    };
    let choices_list = match choices {
        Some(Value::List(l)) => l,
        _ => return (None, None),
    };
    let first = match choices_list.borrow().first().cloned() {
        Some(v) => v,
        None => return (None, None),
    };
    let delta = match &first {
        Value::Map(m) => m.borrow().get("delta").cloned(),
        _ => return (None, None),
    };
    match delta {
        Some(Value::Map(m)) => {
            let m = m.borrow();
            let content = match m.get("content").cloned() {
                Some(Value::Str(s)) => Some(s.to_string()),
                _ => None,
            };
            let reasoning = match m.get("reasoning").cloned() {
                Some(Value::Str(s)) => Some(s.to_string()),
                _ => None,
            };
            (content, reasoning)
        }
        _ => (None, None),
    }
}

// --- Endpoint parsing ---

/// Parse an endpoint URL into (scheme, host, port).
fn parse_endpoint(endpoint: &str) -> Option<(Scheme, String, u16)> {
    let (scheme, rest) = if let Some(r) = endpoint.strip_prefix("https://") {
        (Scheme::Https, r)
    } else if let Some(r) = endpoint.strip_prefix("http://") {
        (Scheme::Http, r)
    } else {
        return None;
    };
    if rest.is_empty() {
        return None;
    }
    let authority = rest.split_once('/').map(|(a, _)| a).unwrap_or(rest);
    let default_port: u16 = match scheme {
        Scheme::Http => 80,
        Scheme::Https => 443,
    };

    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => {
            if let Ok(port) = p.parse::<u16>() {
                (h.to_string(), port)
            } else {
                (authority.to_string(), default_port)
            }
        }
        None => (authority.to_string(), default_port),
    };
    if host.is_empty() {
        return None;
    }
    Some((scheme, host, port))
}

// --- Authority trait impl ---

impl Authority for ProviderAuthority {
    fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
        let map = match params {
            Value::Map(m) => m.clone(),
            _ => return Err("provider.narrow: expected a map of params".into()),
        };
        let m = map.borrow();

        let mut new_models = self.models.clone();
        let mut new_max_tokens = self.max_tokens;
        let mut new_path = self.path.clone();

        if let Some(Value::List(xs)) = m.get("models") {
            let requested: HashSet<String> = xs
                .borrow()
                .iter()
                .map(|x| match x {
                    Value::Str(s) => Ok(s.to_string()),
                    other => Err(format!(
                        "provider.narrow: models must be strings, got {}",
                        other.type_name()
                    )),
                })
                .collect::<Result<_, String>>()?;
            if !self.models.is_empty() {
                new_models = new_models.intersection(&requested).cloned().collect();
                if new_models.is_empty() {
                    return Err("provider.narrow: no models left after intersection".into());
                }
            } else {
                // Parent allows any model; child restricts
                new_models = requested;
            }
        }

        if let Some(Value::Int(mt)) = m.get("max_tokens") {
            let requested = *mt as u64;
            if self.max_tokens > 0 && requested > self.max_tokens {
                return Err(format!(
                    "provider.narrow: max_tokens {} exceeds parent cap {}",
                    requested, self.max_tokens
                ));
            }
            new_max_tokens = requested;
        }

        if let Some(Value::Str(p)) = m.get("path_prefix") {
            if !p.starts_with(&self.path) && self.path != "/v1/chat/completions" {
                return Err(format!(
                    "provider.narrow: path {:?} does not extend current path {:?}",
                    p, self.path
                ));
            }
            new_path = p.to_string();
        }

        Ok(Rc::new(ProviderAuthority {
            endpoint: self.endpoint.clone(),
            scheme: self.scheme.clone(),
            host: self.host.clone(),
            port: self.port,
            models: new_models,
            path: new_path,
            bound_headers: self.bound_headers.clone(),
            max_tokens: new_max_tokens,
            timeout: self.timeout,
        }))
    }

    fn invoke(&self, rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            // chat(messages, [overrides]) -> string content
            "chat" => {
                let body = self.build_request_body(args)?;
                self.do_chat(&body)
            }

            // chat_json(messages, [overrides]) -> parsed JSON response
            "chat_json" => {
                let body = self.build_request_body(args)?;
                self.do_chat_json(&body)
            }

            // stream(messages, handler, [overrides]) -> accumulated string
            "stream" => {
                if args.len() < 2 {
                    return Err("provider.stream: expected messages and handler arguments".into());
                }
                let handler = &args[1];
                // Build body from messages + optional overrides
                let body_args = if args.len() >= 3 {
                    vec![args[0].clone(), args[2].clone()]
                } else {
                    vec![args[0].clone()]
                };
                let body = self.build_request_body(&body_args)?;
                self.do_stream(rt, &body, handler)
            }

            // models() -> list of allowed model names
            "models" => {
                let models: Vec<Value> = self
                    .models
                    .iter()
                    .map(|m| Value::Str(Rc::new(m.clone())))
                    .collect();
                Ok(Value::List(Rc::new(RefCell::new(models))))
            }

            // describe() -> map with capability info
            "describe" => {
                let mut m = HashMap::new();
                m.insert(
                    "endpoint".into(),
                    Value::Str(Rc::new(self.endpoint.clone())),
                );
                m.insert("path".into(), Value::Str(Rc::new(self.path.clone())));
                m.insert(
                    "scheme".into(),
                    Value::Str(Rc::new(
                        match self.scheme {
                            Scheme::Http => "http",
                            Scheme::Https => "https",
                        }
                        .to_string(),
                    )),
                );
                m.insert(
                    "models".into(),
                    Value::List(Rc::new(RefCell::new(
                        self.models
                            .iter()
                            .map(|m| Value::Str(Rc::new(m.clone())))
                            .collect(),
                    ))),
                );
                m.insert("max_tokens".into(), Value::Int(self.max_tokens as i64));
                // Don't expose bound header values — only names
                let header_names: Vec<Value> = self
                    .bound_headers
                    .iter()
                    .map(|(k, _)| Value::Str(Rc::new(k.clone())))
                    .collect();
                m.insert(
                    "bound_header_names".into(),
                    Value::List(Rc::new(RefCell::new(header_names))),
                );
                Ok(Value::Map(Rc::new(RefCell::new(m))))
            }

            _ => Err(format!(
                "provider: no method `{}` (have: chat, chat_json, stream, models, describe)",
                method
            )),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Runtime;

    struct NoopRuntime;

    impl Runtime for NoopRuntime {
        fn invoke(&mut self, _callee: &Value, _args: &[Value]) -> Result<Value, String> {
            Ok(Value::Nil)
        }
    }

    #[test]
    fn parses_http_endpoints() {
        let (scheme, host, port) = parse_endpoint("http://localhost:11434").unwrap();
        assert!(matches!(scheme, Scheme::Http));
        assert_eq!(host, "localhost");
        assert_eq!(port, 11434);

        let (scheme, host, port) = parse_endpoint("http://192.168.1.100:8080").unwrap();
        assert!(matches!(scheme, Scheme::Http));
        assert_eq!(host, "192.168.1.100");
        assert_eq!(port, 8080);

        let (scheme, host, port) = parse_endpoint("http://example.com").unwrap();
        assert!(matches!(scheme, Scheme::Http));
        assert_eq!(host, "example.com");
        assert_eq!(port, 80);

        assert_eq!(parse_endpoint("not-a-url"), None);
    }

    #[test]
    fn parses_https_endpoints() {
        let (scheme, host, port) = parse_endpoint("https://api.cerebras.ai").unwrap();
        assert!(matches!(scheme, Scheme::Https));
        assert_eq!(host, "api.cerebras.ai");
        assert_eq!(port, 443);

        let (scheme, host, port) = parse_endpoint("https://api.cerebras.ai:8443").unwrap();
        assert!(matches!(scheme, Scheme::Https));
        assert_eq!(host, "api.cerebras.ai");
        assert_eq!(port, 8443);

        let (scheme, host, port) = parse_endpoint("https://example.com").unwrap();
        assert!(matches!(scheme, Scheme::Https));
        assert_eq!(host, "example.com");
        assert_eq!(port, 443);
    }

    #[test]
    fn extracts_chat_content_from_openai_response() {
        let response_json = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "Hello from the model!"
                },
                "finish_reason": "stop"
            }]
        }"#;
        let parsed = json::parse_str(response_json).unwrap();
        let content = extract_chat_content(&parsed).unwrap();
        assert_eq!(content, Value::Str(Rc::new("Hello from the model!".into())));
    }

    #[test]
    fn extracts_stream_delta_content() {
        let chunk_json = r#"{
            "choices": [{
                "delta": {
                    "content": "Hello"
                }
            }]
        }"#;
        let chunk = json::parse_str(chunk_json).unwrap();
        let (content, reasoning) = extract_stream_deltas(&chunk);
        assert_eq!(content, Some("Hello".to_string()));
        assert_eq!(reasoning, None);
    }

    #[test]
    fn extracts_stream_delta_reasoning() {
        let chunk_json = r#"{
            "choices": [{
                "delta": {
                    "reasoning": "Let me think..."
                }
            }]
        }"#;
        let chunk = json::parse_str(chunk_json).unwrap();
        let (content, reasoning) = extract_stream_deltas(&chunk);
        assert_eq!(content, None);
        assert_eq!(reasoning, Some("Let me think...".to_string()));
    }

    #[test]
    fn handles_delta_without_content() {
        let chunk_json = r#"{
            "choices": [{
                "delta": {
                    "role": "assistant"
                }
            }]
        }"#;
        let chunk = json::parse_str(chunk_json).unwrap();
        let (content, reasoning) = extract_stream_deltas(&chunk);
        assert!(content.is_none());
        assert!(reasoning.is_none());
    }

    #[test]
    fn extracts_done_chunk() {
        let chunk_json = r#"{
            "choices": [{
                "delta": {},
                "finish_reason": "stop"
            }]
        }"#;
        let chunk = json::parse_str(chunk_json).unwrap();
        let (content, reasoning) = extract_stream_deltas(&chunk);
        assert!(content.is_none());
        assert!(reasoning.is_none());
    }

    #[test]
    fn narrow_restricts_models() {
        let base = ProviderAuthority::new("http://localhost:11434");
        let scoped = ProviderAuthority::with_model(base, "llama3");
        let scoped = ProviderAuthority::with_model(scoped, "gemma2");

        let mut params = HashMap::new();
        params.insert(
            "models".to_string(),
            Value::List(Rc::new(RefCell::new(vec![Value::Str(Rc::new(
                "llama3".into(),
            ))]))),
        );
        let narrowed = scoped
            .narrow(&Value::Map(Rc::new(RefCell::new(params))))
            .unwrap();

        let mut rt = NoopRuntime;
        let models_result = narrowed.invoke(&mut rt, "models", &[]).unwrap();
        match models_result {
            Value::List(xs) => {
                let names: Vec<String> = xs
                    .borrow()
                    .iter()
                    .map(|v| match v {
                        Value::Str(s) => s.to_string(),
                        _ => String::new(),
                    })
                    .collect();
                assert_eq!(names, vec!["llama3"]);
            }
            other => panic!("expected list, got {:?}", other),
        }
    }

    #[test]
    fn narrow_rejects_max_tokens_above_parent() {
        let base = ProviderAuthority::new("http://localhost:11434");
        let mut params1 = HashMap::new();
        params1.insert("max_tokens".to_string(), Value::Int(100));
        let capped = base
            .narrow(&Value::Map(Rc::new(RefCell::new(params1))))
            .unwrap();

        let mut params2 = HashMap::new();
        params2.insert("max_tokens".to_string(), Value::Int(200));
        let result = capped.narrow(&Value::Map(Rc::new(RefCell::new(params2))));
        match result {
            Err(err) => assert!(err.contains("exceeds parent cap")),
            Ok(_) => panic!("expected narrow to fail"),
        }
    }

    #[test]
    fn describe_does_not_expose_bound_headers() {
        let base = ProviderAuthority::new("http://localhost:11434");
        let with_key =
            ProviderAuthority::with_bound_header(base, "Authorization", "Bearer secret-key-12345");
        let mut rt = NoopRuntime;
        let desc = with_key.invoke(&mut rt, "describe", &[]).unwrap();
        match desc {
            Value::Map(m) => {
                let m = m.borrow();
                match m.get("bound_header_names") {
                    Some(Value::List(names)) => {
                        assert_eq!(names.borrow().len(), 1);
                        assert_eq!(
                            names.borrow()[0],
                            Value::Str(Rc::new("Authorization".into()))
                        );
                    }
                    other => panic!("expected list of header names, got {:?}", other),
                }
                let desc_str = format!("{:?}", m);
                assert!(
                    !desc_str.contains("secret-key-12345"),
                    "bound header value leaked into describe output"
                );
            }
            other => panic!("expected map, got {:?}", other),
        }
    }
}
