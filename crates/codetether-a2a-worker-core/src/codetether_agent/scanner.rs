use super::{
    classifier::classify_child_session_value, record::SubAgentRunRecord, state::SubAgentRunState,
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Reads a CodeTether child session JSON file and classifies it.
///
/// # Errors
/// Returns an error when the file cannot be read, parsed, or does not contain CodeTether child-session metadata.
/// # Examples
/// ```rust,no_run
/// use codetether_a2a_worker_core::codetether_agent::subagent_orphans::classify_child_session_file;
/// let record = classify_child_session_file(".codetether-agent/sessions/child.json")?;
/// println!("{}", record.state.as_str());
/// # Ok::<(), String>(())
/// ```
pub fn classify_child_session_file(path: impl AsRef<Path>) -> Result<SubAgentRunRecord, String> {
    let path = path.as_ref();
    let text = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))?;
    classify_child_session_value(&value)
        .ok_or_else(|| format!("{} is not a CodeTether child session", path.display()))
}

/// Scans a CodeTether sessions directory for orphaned child sessions.
/// # Errors
/// Returns an error when the sessions directory cannot be read.
///
/// # Examples
/// ```rust,no_run
/// use codetether_a2a_worker_core::codetether_agent::subagent_orphans::scan_orphaned_child_sessions;
/// let orphans = scan_orphaned_child_sessions(".codetether-agent/sessions")?;
/// println!("{}", orphans.len());
/// # Ok::<(), String>(())
/// ```
pub fn scan_orphaned_child_sessions(
    sessions_dir: impl AsRef<Path>,
) -> Result<Vec<SubAgentRunRecord>, String> {
    let mut records = Vec::new();
    let entries = fs::read_dir(sessions_dir.as_ref())
        .map_err(|error| format!("failed to read sessions dir: {error}"))?;
    for entry in entries {
        let path = entry
            .map_err(|error| format!("failed to read sessions entry: {error}"))?
            .path();
        push_orphan(path, &mut records);
    }
    records.sort_by(|left, right| left.agent_name.cmp(&right.agent_name));
    Ok(records)
}

fn push_orphan(path: PathBuf, records: &mut Vec<SubAgentRunRecord>) {
    if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
        return;
    }
    if let Ok(record) = classify_child_session_file(&path) {
        if record.state == SubAgentRunState::Orphaned {
            records.push(record);
        }
    }
}
