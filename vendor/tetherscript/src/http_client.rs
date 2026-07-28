//! HTTP client: send requests and read responses over plain TCP.
//!
//! Public functions `get`, `head`, `post`, `request` wrap `client_request`.
//! The client uses std TCP plus the crate's platform TLS stream.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::rc::Rc;
use std::time::Duration;

use crate::value::{ResultValue, Value};

use super::http_stream::HttpStream;
use super::http_url::ParsedHttpUrl;

const MAX_START_LINE_BYTES: usize = 8 * 1024;
const MAX_HEADER_LINE_BYTES: usize = 8 * 1024;
const MAX_HEADER_BYTES: usize = 64 * 1024;
const MAX_BODY_BYTES: usize = 16 * 1024 * 1024;
const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);
const CLIENT_USER_AGENT: &str = "tetherscript/0.1 std-http";

/// Send an HTTP GET request. Returns a `Result` value.
pub fn get(url: &Value) -> Value {
    result_value(
        string_arg(url, "http_get: url").and_then(|url| client_request("GET", &url, None, &[])),
    )
}

/// Send an HTTP HEAD request. Returns a `Result` value.
pub fn head(url: &Value) -> Value {
    result_value(
        string_arg(url, "http_head: url").and_then(|url| client_request("HEAD", &url, None, &[])),
    )
}

/// Send an HTTP POST request with a body. Returns a `Result` value.
pub fn post(url: &Value, body: &Value) -> Value {
    let url = string_arg(url, "http_post: url");
    let body = string_arg(body, "http_post: body");
    let result =
        url.and_then(|url| body.and_then(|body| client_request("POST", &url, Some(&body), &[])));
    result_value(result)
}

/// Send an HTTP request with method, url, optional body, and optional headers.
pub fn request(args: &[Value]) -> Value {
    result_value(request_inner(args))
}

fn request_inner(args: &[Value]) -> Result<Value, String> {
    if !(2..=4).contains(&args.len()) {
        return Err("http_request expects method, url[, body[, headers]]".into());
    }

    let method = method_arg(&args[0])?;
    let url = string_arg(&args[1], "http_request: url")?;
    let body = match args.get(2) {
        Some(Value::Nil) | None => None,
        Some(value) => Some(string_arg(value, "http_request: body")?),
    };
    let headers = match args.get(3) {
        Some(value) => headers_arg(value)?,
        None => Vec::new(),
    };

    client_request(&method, &url, body.as_deref(), &headers)
}

fn result_value(result: Result<Value, String>) -> Value {
    Value::Result(Rc::new(match result {
        Ok(value) => ResultValue::Ok(value),
        Err(error) => ResultValue::Err(error),
    }))
}

pub(crate) fn client_request(
    method: &str,
    raw_url: &str,
    body: Option<&str>,
    headers: &[(String, String)],
) -> Result<Value, String> {
    let url = ParsedHttpUrl::parse(raw_url)?;
    let body_bytes = body.map(str::as_bytes).unwrap_or(&[]);
    if body_bytes.len() > MAX_BODY_BYTES {
        return Err(format!(
            "http_request: body exceeds {} bytes",
            MAX_BODY_BYTES
        ));
    }

    let mut stream = super::http_stream::connect(&url, CLIENT_TIMEOUT)?;

    write!(
        stream,
        "{} {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nAccept: */*\r\nConnection: close\r\n",
        method, url.target, url.host_header, CLIENT_USER_AGENT
    )
    .map_err(|e| format!("http_request: write request failed: {}", e))?;

    for (name, value) in headers {
        write!(stream, "{}: {}\r\n", name, value)
            .map_err(|e| format!("http_request: write header failed: {}", e))?;
    }

    if !body_bytes.is_empty() {
        write!(stream, "Content-Length: {}\r\n", body_bytes.len())
            .map_err(|e| format!("http_request: write content-length failed: {}", e))?;
    }

    stream
        .write_all(b"\r\n")
        .and_then(|_| stream.write_all(body_bytes))
        .and_then(|_| stream.flush())
        .map_err(|e| format!("http_request: write body failed: {}", e))?;

    let mut reader = BufReader::new(stream);
    read_client_response(&mut reader, method)
}

fn read_client_response(
    reader: &mut BufReader<Box<dyn HttpStream>>,
    method: &str,
) -> Result<Value, String> {
    let line = read_line_limited(reader, MAX_START_LINE_BYTES, "response status-line")?
        .ok_or_else(|| "http_request: empty response".to_string())?;
    let status_line = line.trim_end_matches(['\r', '\n']);
    let mut parts = status_line.splitn(3, ' ');
    let version = parts.next().unwrap_or_default();
    if !version.starts_with("HTTP/") {
        return Err(format!(
            "http_request: invalid response status-line {}",
            status_line
        ));
    }
    let status: u16 = parts
        .next()
        .ok_or_else(|| "http_request: missing response status".to_string())?
        .parse()
        .map_err(|_| format!("http_request: invalid response status {}", status_line))?;
    let reason = parts.next().unwrap_or_default().to_string();

    let headers = read_response_headers(reader)?;
    let body = if method == "HEAD" {
        Vec::new()
    } else if headers
        .get("transfer-encoding")
        .map(|value| has_token(value, "chunked"))
        .unwrap_or(false)
    {
        read_chunked_body(reader)?
    } else if let Some(content_length) = headers.get("content-length") {
        read_content_length_body(reader, content_length)?
    } else {
        read_close_delimited_body(reader)?
    };

    Ok(client_response_value(status, reason, headers, body))
}

fn read_response_headers(
    reader: &mut BufReader<Box<dyn HttpStream>>,
) -> Result<HashMap<String, String>, String> {
    let mut headers = HashMap::new();
    let mut header_bytes = 0;
    loop {
        let Some(line) = read_line_limited(reader, MAX_HEADER_LINE_BYTES, "response header")?
        else {
            break;
        };
        header_bytes += line.len();
        if header_bytes > MAX_HEADER_BYTES {
            return Err(format!(
                "http_request: response headers exceed {} bytes",
                MAX_HEADER_BYTES
            ));
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }
        if let Some((name, value)) = trimmed.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }
    Ok(headers)
}

fn read_content_length_body(
    reader: &mut BufReader<Box<dyn HttpStream>>,
    content_length: &str,
) -> Result<Vec<u8>, String> {
    let len: usize = content_length
        .parse()
        .map_err(|_| format!("http_request: invalid content-length {}", content_length))?;
    if len > MAX_BODY_BYTES {
        return Err(format!(
            "http_request: content-length {} exceeds {} bytes",
            len, MAX_BODY_BYTES
        ));
    }

    let mut body = vec![0; len];
    if len > 0 {
        reader
            .read_exact(&mut body)
            .map_err(|e| format!("http_request: read body failed: {}", e))?;
    }
    Ok(body)
}

fn read_close_delimited_body(
    reader: &mut BufReader<Box<dyn HttpStream>>,
) -> Result<Vec<u8>, String> {
    let mut body = Vec::new();
    reader
        .take((MAX_BODY_BYTES + 1) as u64)
        .read_to_end(&mut body)
        .map_err(|e| format!("http_request: read body failed: {}", e))?;
    if body.len() > MAX_BODY_BYTES {
        return Err(format!(
            "http_request: response body exceeds {} bytes",
            MAX_BODY_BYTES
        ));
    }
    Ok(body)
}

fn read_chunked_body(reader: &mut BufReader<Box<dyn HttpStream>>) -> Result<Vec<u8>, String> {
    let mut body = Vec::new();
    loop {
        let line = read_line_limited(reader, MAX_HEADER_LINE_BYTES, "chunk size")?
            .ok_or_else(|| "http_request: unexpected EOF reading chunk size".to_string())?;
        let size_text = line
            .trim_end_matches(['\r', '\n'])
            .split(';')
            .next()
            .unwrap_or_default()
            .trim();
        let size = usize::from_str_radix(size_text, 16)
            .map_err(|_| format!("http_request: invalid chunk size {}", size_text))?;

        if size == 0 {
            drain_trailing_headers(reader)?;
            break;
        }
        if body.len() + size > MAX_BODY_BYTES {
            return Err(format!(
                "http_request: response body exceeds {} bytes",
                MAX_BODY_BYTES
            ));
        }

        let start = body.len();
        body.resize(start + size, 0);
        reader
            .read_exact(&mut body[start..])
            .map_err(|e| format!("http_request: read chunk failed: {}", e))?;

        let mut crlf = [0; 2];
        reader
            .read_exact(&mut crlf)
            .map_err(|e| format!("http_request: read chunk terminator failed: {}", e))?;
        if crlf != *b"\r\n" {
            return Err("http_request: invalid chunk terminator".into());
        }
    }
    Ok(body)
}

fn drain_trailing_headers(reader: &mut BufReader<Box<dyn HttpStream>>) -> Result<(), String> {
    let mut bytes = 0;
    loop {
        let Some(line) = read_line_limited(reader, MAX_HEADER_LINE_BYTES, "trailer header")? else {
            return Ok(());
        };
        bytes += line.len();
        if bytes > MAX_HEADER_BYTES {
            return Err(format!(
                "http_request: trailer headers exceed {} bytes",
                MAX_HEADER_BYTES
            ));
        }
        if line.trim_end_matches(['\r', '\n']).is_empty() {
            return Ok(());
        }
    }
}

fn client_response_value(
    status: u16,
    reason: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
) -> Value {
    let headers = headers
        .into_iter()
        .map(|(name, value)| (name, Value::Str(Rc::new(value))))
        .collect();
    let mut response = HashMap::new();
    response.insert("status".into(), Value::Int(status as i64));
    response.insert("ok".into(), Value::Bool((200..300).contains(&status)));
    response.insert("reason".into(), Value::Str(Rc::new(reason)));
    response.insert("headers".into(), Value::Map(Rc::new(RefCell::new(headers))));
    response.insert(
        "body".into(),
        Value::Str(Rc::new(String::from_utf8_lossy(&body).into_owned())),
    );
    Value::Map(Rc::new(RefCell::new(response)))
}

pub(super) fn string_arg(value: &Value, label: &str) -> Result<String, String> {
    match value {
        Value::Str(value) => Ok((**value).clone()),
        other => Err(format!("{} must be str, got {}", label, other.type_name())),
    }
}

fn method_arg(value: &Value) -> Result<String, String> {
    let method = string_arg(value, "http_request: method")?.to_ascii_uppercase();
    if method.is_empty() || !method.bytes().all(is_http_token_byte) {
        return Err(format!("http_request: invalid method {}", method));
    }
    Ok(method)
}

fn headers_arg(value: &Value) -> Result<Vec<(String, String)>, String> {
    let map = match value {
        Value::Map(map) => map.borrow(),
        other => {
            return Err(format!(
                "http_request: headers must be map, got {}",
                other.type_name()
            ))
        }
    };

    let mut headers = Vec::new();
    for (name, value) in map.iter() {
        let name_lower = name.to_ascii_lowercase();
        if matches!(
            name_lower.as_str(),
            "host" | "content-length" | "connection" | "transfer-encoding"
        ) {
            return Err(format!(
                "http_request: header {} is managed by TetherScript",
                name
            ));
        }
        if name.is_empty() || !name.bytes().all(is_http_token_byte) {
            return Err(format!("http_request: invalid header name {}", name));
        }
        let value = match value {
            Value::Str(value) => (**value).clone(),
            other => other.to_string(),
        };
        if value.contains('\r') || value.contains('\n') {
            return Err(format!(
                "http_request: header {} must not contain CR or LF",
                name
            ));
        }
        headers.push((name.clone(), value));
    }
    Ok(headers)
}

fn is_http_token_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'0'..=b'9'
            | b'A'..=b'Z'
            | b'a'..=b'z'
            | b'!'
            | b'#'
            | b'$'
            | b'%'
            | b'&'
            | b'\''
            | b'*'
            | b'+'
            | b'-'
            | b'.'
            | b'^'
            | b'_'
            | b'`'
            | b'|'
            | b'~'
    )
}

fn has_token(value: &str, token: &str) -> bool {
    value
        .split(',')
        .any(|part| part.trim().eq_ignore_ascii_case(token))
}

fn read_line_limited(
    reader: &mut BufReader<Box<dyn HttpStream>>,
    limit: usize,
    label: &str,
) -> Result<Option<String>, String> {
    let mut out = Vec::new();
    loop {
        let available = reader.fill_buf().map_err(|e| {
            if e.kind() == std::io::ErrorKind::WouldBlock
                || e.kind() == std::io::ErrorKind::TimedOut
            {
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
