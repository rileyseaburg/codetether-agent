//! Blocking HTTP/1.1 server: accept loop and request parsing.
//!
//! Scripts call `http_serve(port, handler)`. For each connection we parse
//! a request into a TetherScript Value (a Map with `method`, `path`,
//! `query`, `headers`, `body`), hand it to `handler` via the Runtime
//! bridge, then serialize whatever the handler returns back onto the socket.
//!
//! No Tokio and no threads yet. Supports HTTP keep-alive on each accepted
//! socket with a strict request line + Content-Length body.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::time::Duration;

use crate::value::{Runtime, Value};

use super::http_headers;
use super::http_response;

const MAX_START_LINE_BYTES: usize = 8 * 1024;
const MAX_HEADER_LINE_BYTES: usize = 8 * 1024;
const MAX_HEADER_BYTES: usize = 64 * 1024;
const MAX_BODY_BYTES: usize = 1024 * 1024;
const SERVER_KEEP_ALIVE_IDLE_TIMEOUT: Duration = Duration::from_millis(2);

/// Bind to `0.0.0.0:port` and serve HTTP requests via a TetherScript handler.
///
/// Blocks the calling thread indefinitely. Each incoming connection is
/// handled synchronously with keep-alive support.
pub fn serve(rt: &mut dyn Runtime, port: &Value, handler: &Value) -> Result<Value, String> {
    let port = match port {
        Value::Int(n) if *n > 0 && *n <= 65535 => *n as u16,
        Value::Int(n) => return Err(format!("http_serve: port {} out of range", n)),
        other => {
            return Err(format!(
                "http_serve: port must be int, got {}",
                other.type_name()
            ))
        }
    };

    if !matches!(handler, Value::Fn(_) | Value::VmFn(_) | Value::Native(_)) {
        return Err(format!(
            "http_serve: handler must be a function, got {}",
            handler.type_name()
        ));
    }

    let addr = ("0.0.0.0", port);
    let listener = TcpListener::bind(addr)
        .map_err(|e| format!("http_serve: bind to 0.0.0.0:{} failed: {}", port, e))?;
    eprintln!(
        "tetherscript http: listening on http://0.0.0.0:{} (try http://localhost:{})",
        port, port
    );

    for conn in listener.incoming() {
        let stream = match conn {
            Ok(s) => s,
            Err(e) => {
                eprintln!("tetherscript http: accept error: {}", e);
                continue;
            }
        };
        if let Err(e) = handle_one(rt, handler, stream) {
            eprintln!("tetherscript http: {}", e);
        }
    }
    Ok(Value::Nil)
}

fn handle_one(rt: &mut dyn Runtime, handler: &Value, stream: TcpStream) -> Result<(), String> {
    stream
        .set_read_timeout(Some(SERVER_KEEP_ALIVE_IDLE_TIMEOUT))
        .map_err(|e| format!("set keep-alive timeout: {e}"))?;
    let mut reader = BufReader::new(stream);
    loop {
        let request = match parse_request(&mut reader) {
            Ok(r) => r,
            Err(e) if e == "empty request" => return Ok(()),
            Err(e) => return write_parse_error(reader.get_mut(), e),
        };
        let keep_alive = !http_headers::wants_close(&request);
        let response = match rt.invoke(handler, &[request]) {
            Ok(v) => v,
            Err(e) => return write_handler_error(reader.get_mut(), e),
        };
        http_response::write_response(reader.get_mut(), &response, keep_alive)?;
        if !keep_alive {
            return Ok(());
        }
    }
}

fn write_parse_error(stream: &mut TcpStream, error: String) -> Result<(), String> {
    let _ = http_response::write_simple(
        stream,
        400,
        "text/plain; charset=utf-8",
        error.as_bytes(),
        false,
    );
    Err(format!("parse: {}", error))
}

fn write_handler_error(stream: &mut TcpStream, error: String) -> Result<(), String> {
    let body = format!("handler error: {}", error);
    let _ = http_response::write_simple(
        stream,
        500,
        "text/plain; charset=utf-8",
        body.as_bytes(),
        false,
    );
    Err(format!("handler: {}", error))
}

fn parse_request(reader: &mut BufReader<TcpStream>) -> Result<Value, String> {
    let line = read_line_limited(reader, MAX_START_LINE_BYTES, "start-line")?
        .ok_or_else(|| "empty request".to_string())?;
    let start = line.trim_end_matches(['\r', '\n']).to_string();
    let mut parts = start.splitn(3, ' ');
    let method = parts.next().ok_or("missing method")?.to_string();
    let target = parts.next().ok_or("missing target")?.to_string();
    let _version = parts.next().unwrap_or("HTTP/1.1");

    let (path, query) = match target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (target, String::new()),
    };

    let mut headers: HashMap<String, Value> = HashMap::new();
    let mut content_length: usize = 0;
    let mut header_bytes: usize = 0;
    loop {
        let Some(h) = read_line_limited(reader, MAX_HEADER_LINE_BYTES, "header")? else {
            break;
        };
        header_bytes += h.len();
        if header_bytes > MAX_HEADER_BYTES {
            return Err(format!("headers exceed {} bytes", MAX_HEADER_BYTES));
        }
        let trimmed = h.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }
        if let Some((name, value)) = trimmed.split_once(':') {
            let name = name.trim().to_ascii_lowercase();
            let value = value.trim().to_string();
            if name == "content-length" {
                content_length = value.parse().unwrap_or(0);
                if content_length > MAX_BODY_BYTES {
                    return Err(format!(
                        "content-length {} exceeds {} bytes",
                        content_length, MAX_BODY_BYTES
                    ));
                }
            }
            headers.insert(name, Value::Str(Rc::new(value)));
        }
    }

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .map_err(|e| format!("read body: {}", e))?;
    }
    let body_str = String::from_utf8_lossy(&body).into_owned();

    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert("method".into(), Value::Str(Rc::new(method)));
    map.insert("path".into(), Value::Str(Rc::new(path)));
    map.insert("query".into(), Value::Str(Rc::new(query)));
    map.insert("headers".into(), Value::Map(Rc::new(RefCell::new(headers))));
    map.insert("body".into(), Value::Str(Rc::new(body_str)));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn read_line_limited(
    reader: &mut BufReader<TcpStream>,
    limit: usize,
    label: &str,
) -> Result<Option<String>, String> {
    let mut out = Vec::new();
    loop {
        let available = reader.fill_buf().map_err(|e| {
            if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut {
                "empty request".to_string()
            } else {
                format!("read {label}: {e}")
            }
        })?;
        if available.is_empty() {
            if out.is_empty() {
                return Ok(None);
            }
            break;
        }

        let take_len = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |pos| pos + 1);
        if out.len() + take_len > limit {
            return Err(format!("{label} exceeds {limit} bytes"));
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
        .map_err(|e| format!("read {label}: invalid UTF-8: {e}"))
}
