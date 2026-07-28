use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use tetherscript::browser_cap::BrowserAuthority;
use tetherscript::capability::Authority;
use tetherscript::value::{Runtime, Value};

struct NoopRuntime;

impl Runtime for NoopRuntime {
    fn invoke(&mut self, _callee: &Value, _args: &[Value]) -> Result<Value, String> {
        Ok(Value::Nil)
    }
}

struct FixtureServer {
    url: String,
    stop: Arc<AtomicBool>,
    handle: thread::JoinHandle<()>,
}

impl FixtureServer {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap());
        let stop = Arc::new(AtomicBool::new(false));
        let stop_thread = stop.clone();
        let handle = thread::spawn(move || serve_fixture(listener, stop_thread));
        Self { url, stop, handle }
    }

    fn stop(self) {
        self.stop.store(true, Ordering::SeqCst);
        self.handle.join().unwrap();
    }
}

fn serve_fixture(listener: TcpListener, stop: Arc<AtomicBool>) {
    while !stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let mut buf = [0; 1024];
                let _ = stream.read(&mut buf);
                let body = "<!doctype html><button id=ready>Agent Browser Smoke</button>";
                let reply = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(reply.as_bytes());
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(25));
            }
            Err(_) => break,
        }
    }
}

fn str_value(text: &str) -> Value {
    Value::Str(Rc::new(text.into()))
}

fn invoke(auth: &Rc<dyn Authority>, method: &str, args: &[Value]) -> Result<Value, String> {
    auth.invoke(&mut NoopRuntime, method, args)
}

#[test]
fn live_browserctl_smoke_when_endpoint_is_configured() {
    let Ok(endpoint) = env::var("TETHERSCRIPT_BROWSERCTL_ENDPOINT") else {
        return;
    };
    let fixture = FixtureServer::start();
    let auth = BrowserAuthority::new(
        &endpoint,
        vec![fixture.url.clone()],
        BrowserAuthority::all_scopes(),
    );
    let shot = env::temp_dir().join("tetherscript-browserctl-smoke.png");
    let mut errors = Vec::new();

    for (method, args) in [
        ("start", vec![]),
        ("goto", vec![str_value(&fixture.url)]),
        ("wait_for_text", vec![str_value("Agent Browser Smoke")]),
        (
            "eval",
            vec![str_value("document.querySelector('#ready').textContent")],
        ),
        ("page_snapshot", vec![]),
        ("screenshot", vec![str_value(&shot.display().to_string())]),
    ] {
        if let Err(err) = invoke(&auth, method, &args) {
            errors.push(format!("browser.{method}: {err}"));
            break;
        }
    }

    let _ = invoke(&auth, "stop", &[]);
    fixture.stop();
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}
