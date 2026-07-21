//! Deserialized mux control arguments.

#[derive(serde::Deserialize)]
pub(super) struct Args {
    pub action: String,
    pub name: Option<String>,
    pub message: Option<String>,
    pub workspace: Option<std::path::PathBuf>,
    pub session_id: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
    #[serde(default)]
    pub force: bool,
}

fn default_timeout() -> u64 {
    10_000
}
