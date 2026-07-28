//! Language Server Protocol server for TetherScript.
//!
//! A minimal LSP implementation: lifecycle, full-text sync, and diagnostics
//! published from lex / parse errors. Speaks JSON-RPC 2.0 over stdio with
//! LSP's Content-Length framing. JSON is handled by TetherScript's dependency-free
//! parser/encoder.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::rc::Rc;

use crate::json;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::Value;

pub fn run() -> io::Result<()> {
    let mut docs: HashMap<String, String> = HashMap::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    loop {
        let msg = match read_message(&mut stdin)? {
            Some(m) => m,
            None => return Ok(()),
        };

        let id = field_value(&msg, "id");
        let method = field_value(&msg, "method")
            .and_then(|value| as_str(&value).map(str::to_string))
            .unwrap_or_default();
        let params = field_value(&msg, "params").unwrap_or(Value::Nil);

        if let Some(id) = id {
            handle_request(id, &method, &params)?;
        } else {
            match method.as_str() {
                "exit" => return Ok(()),
                _ => handle_notification(&method, &params, &mut docs)?,
            }
        }
    }
}

fn handle_request(id: Value, method: &str, _params: &Value) -> io::Result<()> {
    match method {
        "initialize" => send_response(
            id,
            Some(obj(vec![
                (
                    "capabilities",
                    obj(vec![("textDocumentSync", Value::Int(1))]),
                ),
                (
                    "serverInfo",
                    obj(vec![
                        ("name", str_value("tetherscript-lsp")),
                        ("version", str_value(env!("CARGO_PKG_VERSION"))),
                    ]),
                ),
            ])),
            None,
        ),
        "shutdown" => send_response(id, Some(Value::Nil), None),
        _ => send_response(
            id,
            None,
            Some(obj(vec![
                ("code", Value::Int(-32601)),
                (
                    "message",
                    str_value(&format!("method not found: {}", method)),
                ),
            ])),
        ),
    }
}

fn handle_notification(
    method: &str,
    params: &Value,
    docs: &mut HashMap<String, String>,
) -> io::Result<()> {
    match method {
        "initialized" => Ok(()),
        "textDocument/didOpen" => {
            let uri = pointer_value(params, &["textDocument", "uri"])
                .and_then(|value| as_str(&value).map(str::to_string))
                .unwrap_or_default();
            let text = pointer_value(params, &["textDocument", "text"])
                .and_then(|value| as_str(&value).map(str::to_string))
                .unwrap_or_default();
            docs.insert(uri.clone(), text.clone());
            publish_diagnostics(&uri, &text)
        }
        "textDocument/didChange" => {
            let uri = pointer_value(params, &["textDocument", "uri"])
                .and_then(|value| as_str(&value).map(str::to_string))
                .unwrap_or_default();
            if let Some(changes) = pointer_value(params, &["contentChanges"]).and_then(as_list) {
                if let Some(last) = changes.borrow().last() {
                    if let Some(text) = field_value(last, "text")
                        .and_then(|value| as_str(&value).map(str::to_string))
                    {
                        docs.insert(uri.clone(), text.to_string());
                        return publish_diagnostics(&uri, &text);
                    }
                }
            }
            Ok(())
        }
        "textDocument/didSave" => {
            let uri = pointer_value(params, &["textDocument", "uri"])
                .and_then(|value| as_str(&value).map(str::to_string))
                .unwrap_or_default();
            if let Some(text) = docs.get(&uri).cloned() {
                return publish_diagnostics(&uri, &text);
            }
            Ok(())
        }
        "textDocument/didClose" => {
            let uri = pointer_value(params, &["textDocument", "uri"])
                .and_then(|value| as_str(&value).map(str::to_string))
                .unwrap_or_default();
            docs.remove(&uri);
            send_notification(
                "textDocument/publishDiagnostics",
                obj(vec![
                    ("uri", str_value(&uri)),
                    (
                        "diagnostics",
                        Value::List(Rc::new(RefCell::new(Vec::new()))),
                    ),
                ]),
            )
        }
        _ => Ok(()),
    }
}

fn publish_diagnostics(uri: &str, text: &str) -> io::Result<()> {
    send_notification(
        "textDocument/publishDiagnostics",
        obj(vec![
            ("uri", str_value(uri)),
            ("diagnostics", compute_diagnostics(text)),
        ]),
    )
}

fn compute_diagnostics(text: &str) -> Value {
    let mut diags = Vec::new();
    match Lexer::new(text).tokenize() {
        Err(e) => diags.push(diag(e.line, e.col, "lex error", &e.msg)),
        Ok(tokens) => {
            if let Err(e) = Parser::new(tokens).parse_program() {
                diags.push(diag(e.line, e.col, "parse error", &e.msg));
            }
        }
    }
    Value::List(Rc::new(RefCell::new(diags)))
}

fn diag(line: usize, col: usize, kind: &str, msg: &str) -> Value {
    let l = line.saturating_sub(1) as i64;
    let c = col.saturating_sub(1) as i64;
    obj(vec![
        (
            "range",
            obj(vec![
                (
                    "start",
                    obj(vec![("line", Value::Int(l)), ("character", Value::Int(c))]),
                ),
                (
                    "end",
                    obj(vec![
                        ("line", Value::Int(l)),
                        ("character", Value::Int(c + 1)),
                    ]),
                ),
            ]),
        ),
        ("severity", Value::Int(1)),
        ("source", str_value("tetherscript")),
        ("message", str_value(&format!("{}: {}", kind, msg))),
    ])
}

fn read_message(stdin: &mut impl BufRead) -> io::Result<Option<Value>> {
    let mut content_length: usize = 0;
    loop {
        let mut line = String::new();
        let n = stdin.read_line(&mut line)?;
        if n == 0 {
            return Ok(None);
        }
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            break;
        }
        if let Some(rest) = line.strip_prefix("Content-Length:") {
            content_length = rest.trim().parse().unwrap_or(0);
        }
    }
    let mut buf = vec![0u8; content_length];
    stdin.read_exact(&mut buf)?;
    let text = String::from_utf8_lossy(&buf);
    Ok(json::parse_str(&text).ok())
}

fn send_message(msg: Value) -> io::Result<()> {
    let body = json::encode_to_string(&msg)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    write!(out, "Content-Length: {}\r\n\r\n{}", body.len(), body)?;
    out.flush()
}

fn send_response(id: Value, result: Option<Value>, error: Option<Value>) -> io::Result<()> {
    let mut fields = vec![("jsonrpc", str_value("2.0")), ("id", id)];
    if let Some(result) = result {
        fields.push(("result", result));
    }
    if let Some(error) = error {
        fields.push(("error", error));
    }
    send_message(obj(fields))
}

fn send_notification(method: &str, params: Value) -> io::Result<()> {
    send_message(obj(vec![
        ("jsonrpc", str_value("2.0")),
        ("method", str_value(method)),
        ("params", params),
    ]))
}

fn obj(fields: Vec<(&str, Value)>) -> Value {
    Value::Map(Rc::new(RefCell::new(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
    )))
}

fn str_value(value: &str) -> Value {
    Value::Str(Rc::new(value.to_string()))
}

fn field_value(value: &Value, name: &str) -> Option<Value> {
    match value {
        Value::Map(map) => map.borrow().get(name).cloned(),
        _ => None,
    }
}

fn pointer_value(value: &Value, path: &[&str]) -> Option<Value> {
    let mut current = value.clone();
    for part in path {
        current = field_value(&current, part)?;
    }
    Some(current)
}

fn as_str(value: &Value) -> Option<&str> {
    match value {
        Value::Str(value) => Some(value.as_str()),
        _ => None,
    }
}

fn as_list(value: Value) -> Option<Rc<RefCell<Vec<Value>>>> {
    match value {
        Value::List(value) => Some(value),
        _ => None,
    }
}
