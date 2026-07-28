use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

use tetherscript::browser_cap::BrowserAuthority;
use tetherscript::capability::Authority;
use tetherscript::value::{Runtime, Value};

struct NoopRuntime;

impl Runtime for NoopRuntime {
    fn invoke(&mut self, _callee: &Value, _args: &[Value]) -> Result<Value, String> {
        Ok(Value::Nil)
    }
}

struct FakeBridge {
    endpoint: String,
    request: Arc<Mutex<String>>,
    body: Arc<Mutex<String>>,
    handle: thread::JoinHandle<()>,
}

impl FakeBridge {
    fn new(response: &'static str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}/browser", listener.local_addr().unwrap());
        let request = Arc::new(Mutex::new(String::new()));
        let body = Arc::new(Mutex::new(String::new()));
        let request_clone = request.clone();
        let body_clone = body.clone();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut data = Vec::new();
            let mut buf = [0; 4096];
            loop {
                let n = stream.read(&mut buf).unwrap();
                assert!(n > 0, "bridge request closed before headers");
                data.extend_from_slice(&buf[..n]);
                if String::from_utf8_lossy(&data).contains("\r\n\r\n") {
                    break;
                }
            }
            let mut text = String::from_utf8_lossy(&data).into_owned();
            let split = text.find("\r\n\r\n").unwrap() + 4;
            let content_length = text[..split]
                .lines()
                .find_map(|line| line.strip_prefix("Content-Length: "))
                .unwrap()
                .parse::<usize>()
                .unwrap();
            while text[split..].len() < content_length {
                let n = stream.read(&mut buf).unwrap();
                assert!(n > 0, "bridge request closed before body");
                data.extend_from_slice(&buf[..n]);
                text = String::from_utf8_lossy(&data).into_owned();
            }
            *request_clone.lock().unwrap() = text[..split].to_string();
            *body_clone.lock().unwrap() = text[split..split + content_length].to_string();
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                response.len(),
                response
            );
            stream.write_all(reply.as_bytes()).unwrap();
        });
        Self {
            endpoint,
            request,
            body,
            handle,
        }
    }
}

fn str_value(text: &str) -> Value {
    Value::Str(Rc::new(text.into()))
}

fn map(entries: Vec<(&str, Value)>) -> Value {
    Value::Map(Rc::new(RefCell::new(
        entries.into_iter().map(|(k, v)| (k.into(), v)).collect(),
    )))
}

fn list(values: Vec<Value>) -> Value {
    Value::List(Rc::new(RefCell::new(values)))
}

fn invoke(auth: &Rc<dyn Authority>, method: &str, args: &[Value]) -> Result<Value, String> {
    auth.invoke(&mut NoopRuntime, method, args)
}

fn all_scopes() -> Vec<String> {
    [
        "browser.navigate",
        "browser.interact",
        "browser.inspect.dom",
        "browser.inspect.network",
        "browser.inspect.console",
        "browser.inspect.storage",
        "browser.inspect.react",
        "browser.mutate.storage",
        "browser.replay.network",
        "browser.screenshot",
        "browser.visual",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn body_action(body: &str) -> String {
    let needle = "\"action\":\"";
    let start = body.find(needle).unwrap() + needle.len();
    let rest = &body[start..];
    rest[..rest.find('"').unwrap()].to_string()
}

fn is_browserctl_action(action: &str) -> bool {
    matches!(
        action,
        "health"
            | "detect"
            | "start"
            | "stop"
            | "snapshot"
            | "goto"
            | "back"
            | "click"
            | "upload"
            | "fill"
            | "type"
            | "press"
            | "hover"
            | "focus"
            | "blur"
            | "scroll"
            | "text"
            | "html"
            | "eval"
            | "click_text"
            | "fill_native"
            | "toggle"
            | "screenshot"
            | "mouse_click"
            | "keyboard_type"
            | "keyboard_press"
            | "reload"
            | "wait"
            | "tabs"
            | "tabs_select"
            | "tabs_new"
            | "tabs_close"
            | "network_log"
            | "fetch"
            | "axios"
            | "xhr"
            | "replay"
            | "diagnose"
    )
}

#[test]
fn goto_posts_browserctl_action_envelope_to_bridge_root() {
    let bridge = FakeBridge::new("{\"ok\":true,\"value\":{\"status\":\"ok\"}}");
    let auth = BrowserAuthority::new(
        &bridge.endpoint,
        vec!["http://app.test".into()],
        vec!["browser.navigate".into()],
    );

    invoke(&auth, "goto", &[str_value("http://app.test/home")]).unwrap();
    bridge.handle.join().unwrap();

    assert!(bridge
        .request
        .lock()
        .unwrap()
        .starts_with("POST /browser HTTP/1.1"));
    let body = bridge.body.lock().unwrap();
    assert!(body.contains("\"action\":\"goto\""));
    assert!(body.contains("\"url\":\"http://app.test/home\""));
}

#[test]
fn raw_actions_are_action_enveloped_and_scope_checked() {
    let bridge = FakeBridge::new("{\"ok\":true,\"value\":[]}");
    let auth = BrowserAuthority::new(
        &bridge.endpoint,
        Vec::new(),
        vec!["browser.inspect.network".into()],
    );

    invoke(
        &auth,
        "raw",
        &[
            str_value("network_log"),
            map(vec![("url_contains", str_value("/api"))]),
        ],
    )
    .unwrap();
    bridge.handle.join().unwrap();

    let body = bridge.body.lock().unwrap();
    assert!(body.contains("\"action\":\"network_log\""));
    assert!(body.contains("\"url_contains\":\"/api\""));
}

#[test]
fn tool_result_output_json_is_normalized_to_value() {
    let bridge = FakeBridge::new("{\"success\":true,\"output\":\"{\\\"title\\\":\\\"ok\\\"}\"}");
    let auth = BrowserAuthority::new(
        &bridge.endpoint,
        Vec::new(),
        vec!["browser.inspect.dom".into()],
    );

    let value = invoke(&auth, "page_snapshot", &[]).unwrap();
    bridge.handle.join().unwrap();

    match value {
        Value::Map(m) => assert_eq!(m.borrow().get("title"), Some(&str_value("ok"))),
        other => panic!("expected map value, got {}", other.type_name()),
    }
}

#[test]
fn missing_scope_denies_before_network_io() {
    let auth = BrowserAuthority::new("http://127.0.0.1:1/browser", Vec::new(), Vec::new());
    let err = invoke(&auth, "goto", &[str_value("http://app.test")]).unwrap_err();
    assert!(err.contains("scope `browser.navigate` not granted"));
}

#[test]
fn origin_denial_happens_before_network_io() {
    let auth = BrowserAuthority::new(
        "http://127.0.0.1:1/browser",
        vec!["http://allowed.test".into()],
        vec!["browser.navigate".into()],
    );
    let err = invoke(&auth, "goto", &[str_value("http://evil.test")]).unwrap_err();
    assert!(err.contains("origin http://evil.test is not granted"));
}

#[test]
fn high_level_methods_emit_only_browserctl_actions() {
    let cases = vec![
        ("health", vec![]),
        ("detect", vec![]),
        ("start", vec![]),
        ("stop", vec![]),
        ("goto", vec![str_value("http://app.test/home")]),
        ("reload", vec![]),
        ("back", vec![]),
        ("tabs", vec![]),
        ("tabs_select", vec![Value::Int(0)]),
        ("tabs_new", vec![str_value("http://app.test/new")]),
        ("tabs_close", vec![Value::Int(0)]),
        ("wait_for_url", vec![str_value("/ready")]),
        ("click", vec![str_value("#save")]),
        (
            "upload",
            vec![
                str_value("input[type=file]"),
                list(vec![str_value("a.txt")]),
            ],
        ),
        (
            "fill",
            vec![str_value("#email"), str_value("riley@example.com")],
        ),
        ("type", vec![str_value("#email"), str_value("abc")]),
        ("press", vec![str_value("Enter")]),
        ("hover", vec![str_value("#save")]),
        ("focus", vec![str_value("#email")]),
        ("blur", vec![str_value("#email")]),
        ("scroll", vec![str_value("#panel")]),
        ("scroll", vec![Value::Int(0), Value::Int(400)]),
        ("click_text", vec![str_value("Save")]),
        ("fill_native", vec![str_value("#email")]),
        ("toggle", vec![str_value("#enabled")]),
        ("mouse_click", vec![Value::Int(1), Value::Int(2)]),
        ("keyboard_type", vec![str_value("hello")]),
        ("keyboard_press", vec![str_value("Enter")]),
        ("snapshot", vec![]),
        ("page_snapshot", vec![]),
        ("dom_snapshot", vec![]),
        ("text", vec![str_value("body")]),
        ("html", vec![str_value("body")]),
        ("eval", vec![str_value("document.title")]),
        ("wait_for_selector", vec![str_value("#ready")]),
        ("wait_for_text", vec![str_value("Ready")]),
        ("screenshot", vec![str_value("screen.png")]),
        ("console_logs", vec![]),
        ("react.detect", vec![]),
        ("network_log", vec![str_value("/api")]),
        ("failed_requests", vec![]),
        ("fetch", vec![str_value("http://app.test/api")]),
        ("axios", vec![str_value("http://app.test/api")]),
        ("xhr", vec![str_value("http://app.test/api")]),
        ("replay", vec![str_value("/api")]),
        ("replay_request", vec![str_value("/api")]),
        ("diagnose", vec![]),
        ("wait_for_request", vec![str_value("/api")]),
        ("wait_for_response", vec![str_value("/api")]),
        ("cookies", vec![]),
        ("local_storage", vec![]),
        ("session_storage", vec![]),
        ("indexed_db_summary", vec![]),
        ("set_cookie", vec![str_value("a=b")]),
        ("set_local_storage", vec![str_value("k"), str_value("v")]),
        ("clear_storage", vec![]),
        ("is_visible", vec![str_value("#save")]),
        ("is_enabled", vec![str_value("#save")]),
        ("bounding_box", vec![str_value("#save")]),
        ("screenshot_element", vec![str_value("#save")]),
        ("find_visual_text", vec![str_value("Save")]),
        ("find_element_at", vec![Value::Int(1), Value::Int(2)]),
    ];

    for (method, args) in cases {
        let bridge = FakeBridge::new("{\"ok\":true,\"value\":null}");
        let auth = BrowserAuthority::new(
            &bridge.endpoint,
            vec!["http://app.test".into()],
            all_scopes(),
        );
        invoke(&auth, method, &args)
            .unwrap_or_else(|err| panic!("browser.{method} failed before bridge: {err}"));
        bridge.handle.join().unwrap();
        let action = body_action(&bridge.body.lock().unwrap());
        assert!(
            is_browserctl_action(&action),
            "browser.{method} emitted non-browserctl action `{action}`"
        );
    }
}

#[test]
fn unsupported_backend_methods_fail_before_network_io() {
    let auth = BrowserAuthority::new("http://127.0.0.1:1/browser", Vec::new(), all_scopes());
    let forward = invoke(&auth, "forward", &[]).unwrap_err();
    assert!(forward.contains("does not support forward"));

    let idle = invoke(&auth, "wait_for_network_idle", &[]).unwrap_err();
    assert!(idle.contains("does not support network idle waits"));

    let diff = invoke(
        &auth,
        "visual_diff",
        &[str_value("before.png"), str_value("after.png")],
    )
    .unwrap_err();
    assert!(diff.contains("does not support visual diff actions"));
}
