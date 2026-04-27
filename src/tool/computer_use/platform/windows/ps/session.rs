use once_cell::sync::Lazy;
use tokio::sync::Mutex;

mod io;
mod process;

static SESSION: Lazy<Mutex<Option<process::PsSession>>> = Lazy::new(|| Mutex::new(None));

pub async fn run(script: &str) -> anyhow::Result<serde_json::Value> {
    let mut guard = SESSION.lock().await;
    if guard.is_none() {
        *guard = Some(process::PsSession::start()?);
    }
    guard
        .as_mut()
        .expect("session initialized")
        .run(script)
        .await
}
