//! Best-effort recovery of a legacy working directory from later records.
use codetether_codex_records::extract_cwd_from_env_context;
use serde_json::Value;
pub(crate) fn recover(line: &str) -> Option<String> {
    let value: Value = serde_json::from_str(line).ok()?;
    extract_cwd_from_env_context(&value)
}
