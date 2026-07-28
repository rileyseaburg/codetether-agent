//! Integration tests for the HTTP client and server modules.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::rc::Rc;
use std::thread::{self, JoinHandle};

use super::http_client;
use super::http_url::ParsedHttpUrl;
use crate::compiler::Compiler;
use crate::interp::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::Value;
use crate::vm::VM;

fn spawn_once(response: Vec<u8>) -> (String, JoinHandle<String>) {
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request = String::new();
        loop {
            let mut line = String::new();
            let n = reader.read_line(&mut line).unwrap();
            if n == 0 {
                break;
            }
            request.push_str(&line);
            if line == "\r\n" {
                break;
            }
        }
        stream.write_all(&response).unwrap();
        stream.flush().unwrap();
        request
    });
    (format!("http://{}", addr), handle)
}

#[test]
fn parses_plain_http_urls() {
    assert_eq!(
        ParsedHttpUrl::parse("http://example.com:8080/path?q=1").unwrap(),
        ParsedHttpUrl {
            https: false,
            host: "example.com".into(),
            port: 8080,
            host_header: "example.com:8080".into(),
            target: "/path?q=1".into(),
        }
    );
    assert_eq!(
        ParsedHttpUrl::parse("http://example.com?q=1")
            .unwrap()
            .target,
        "/?q=1"
    );
}

#[test]
fn parses_https_urls() {
    let parsed = ParsedHttpUrl::parse("https://example.com/login").unwrap();
    assert!(parsed.https);
    assert_eq!(parsed.port, 443);
    assert_eq!(parsed.host_header, "example.com");
    assert_eq!(parsed.target, "/login");
}

#[test]
fn get_fetches_plain_http() {
    let (base, handle) = spawn_once(
        b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 18\r\n\r\nhello tetherscript"
            .to_vec(),
    );
    let response =
        http_client::client_request("GET", &format!("{}/demo?x=1", base), None, &[]).unwrap();
    match response {
        Value::Map(map) => {
            let map = map.borrow();
            assert_eq!(map.get("status"), Some(&Value::Int(200)));
            assert_eq!(map.get("ok"), Some(&Value::Bool(true)));
            assert_eq!(
                map.get("body"),
                Some(&Value::Str(Rc::new("hello tetherscript".into())))
            );
        }
        other => panic!("expected response map, got {:?}", other),
    }

    let request = handle.join().unwrap();
    assert!(request.starts_with("GET /demo?x=1 HTTP/1.1\r\n"));
    assert!(request.contains("\r\nHost: 127.0.0.1:"));
}

#[test]
fn chunked_response_is_decoded() {
    let (base, handle) = spawn_once(
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n1\r\n!\r\n0\r\n\r\n"
            .to_vec(),
    );
    let response = http_client::client_request("GET", &base, None, &[]).unwrap();
    match response {
        Value::Map(map) => {
            assert_eq!(
                map.borrow().get("body"),
                Some(&Value::Str(Rc::new("hello!".into())))
            );
        }
        other => panic!("expected response map, got {:?}", other),
    }
    handle.join().unwrap();
}

#[test]
fn interpreter_can_call_http_get_builtin() {
    let (base, handle) =
        spawn_once(b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nfrom builtin".to_vec());
    let source = format!(
        r#"
let resp = http_get("{}").unwrap()
resp.body
"#,
        base
    );
    let tokens = Lexer::new(&source).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    let mut interp = Interpreter::new();
    let result = interp.run_repl(&program).unwrap();

    assert_eq!(result, Value::Str(Rc::new("from builtin".into())));
    handle.join().unwrap();
}

#[test]
fn vm_can_call_http_get_builtin() {
    let (base, handle) =
        spawn_once(b"HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\nfrom vm".to_vec());
    let source = format!(
        r#"
fn main() {{
    let resp = http_get("{}").unwrap()
    if resp.body != "from vm" {{
        panic "bad body"
    }}
}}
"#,
        base
    );
    let tokens = Lexer::new(&source).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    let chunk = Compiler::compile_program(&program);
    let mut vm = VM::new();

    vm.run(chunk).unwrap();
    handle.join().unwrap();
}
