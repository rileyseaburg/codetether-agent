//! PowerShell bridge for Windows GUI operations.

mod escape;
mod parse;
mod session;

pub use escape::escape;

pub async fn run(script: &str) -> anyhow::Result<serde_json::Value> {
    session::run(script).await
}
