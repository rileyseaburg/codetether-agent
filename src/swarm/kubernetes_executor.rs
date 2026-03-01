//! Shared protocol types for Kubernetes-backed swarm execution.

use super::subtask::SubTaskResult;
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const SWARM_SUBTASK_PAYLOAD_ENV: &str = "CODETETHER_SWARM_SUBTASK_PAYLOAD";
pub const SWARM_SUBTASK_PROBE_PREFIX: &str = "CT_SUBTASK_PROBE ";
pub const SWARM_SUBTASK_RESULT_PREFIX: &str = "CT_SUBTASK_RESULT ";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSubtaskPayload {
    pub swarm_id: String,
    pub subtask_id: String,
    pub subtask_name: String,
    pub specialty: String,
    pub instruction: String,
    pub context: String,
    pub provider: String,
    pub model: String,
    pub max_steps: usize,
    pub timeout_secs: u64,
    pub working_dir: Option<String>,
    #[serde(default = "default_probe_interval_secs")]
    pub probe_interval_secs: u64,
}

fn default_probe_interval_secs() -> u64 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBranchProbe {
    pub subtask_id: String,
    pub compile_ok: bool,
    pub changed_files: Vec<String>,
    pub changed_lines: u32,
}

pub fn encode_payload(payload: &RemoteSubtaskPayload) -> Result<String> {
    let json = serde_json::to_vec(payload)?;
    Ok(STANDARD.encode(json))
}

pub fn decode_payload(payload_b64: &str) -> Result<RemoteSubtaskPayload> {
    let bytes = STANDARD
        .decode(payload_b64.as_bytes())
        .map_err(|e| anyhow!("Invalid base64 payload: {e}"))?;
    let payload = serde_json::from_slice::<RemoteSubtaskPayload>(&bytes)
        .map_err(|e| anyhow!("Invalid remote payload JSON: {e}"))?;
    Ok(payload)
}

pub fn latest_probe_from_logs(logs: &str) -> Option<RemoteBranchProbe> {
    logs.lines()
        .rev()
        .find_map(|line| line.strip_prefix(SWARM_SUBTASK_PROBE_PREFIX))
        .and_then(|json| serde_json::from_str::<RemoteBranchProbe>(json).ok())
}

pub fn result_from_logs(logs: &str) -> Option<SubTaskResult> {
    logs.lines()
        .rev()
        .find_map(|line| line.strip_prefix(SWARM_SUBTASK_RESULT_PREFIX))
        .and_then(|json| serde_json::from_str::<SubTaskResult>(json).ok())
}

pub fn probe_changed_files_set(probe: &RemoteBranchProbe) -> HashSet<String> {
    probe.changed_files.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_roundtrip() {
        let payload = RemoteSubtaskPayload {
            swarm_id: "swarm-1".to_string(),
            subtask_id: "subtask-1".to_string(),
            subtask_name: "name".to_string(),
            specialty: "specialist".to_string(),
            instruction: "do work".to_string(),
            context: "ctx".to_string(),
            provider: "zai".to_string(),
            model: "glm-5".to_string(),
            max_steps: 20,
            timeout_secs: 180,
            working_dir: Some("/workspace".to_string()),
            probe_interval_secs: 5,
        };
        let encoded = encode_payload(&payload).expect("encode");
        let decoded = decode_payload(&encoded).expect("decode");
        assert_eq!(decoded.subtask_id, "subtask-1");
        assert_eq!(decoded.provider, "zai");
    }

    #[test]
    fn parse_probe_and_result_from_logs() {
        let probe = RemoteBranchProbe {
            subtask_id: "subtask-1".to_string(),
            compile_ok: true,
            changed_files: vec!["src/main.rs".to_string()],
            changed_lines: 42,
        };
        let result = SubTaskResult {
            subtask_id: "subtask-1".to_string(),
            subagent_id: "agent-subtask-1".to_string(),
            success: true,
            result: "ok".to_string(),
            steps: 3,
            tool_calls: 2,
            execution_time_ms: 1000,
            error: None,
            artifacts: Vec::new(),
            retry_count: 0,
        };
        let logs = format!(
            "line\n{}{}\nline\n{}{}\n",
            SWARM_SUBTASK_PROBE_PREFIX,
            serde_json::to_string(&probe).expect("probe json"),
            SWARM_SUBTASK_RESULT_PREFIX,
            serde_json::to_string(&result).expect("result json"),
        );
        let parsed_probe = latest_probe_from_logs(&logs).expect("probe");
        let parsed_result = result_from_logs(&logs).expect("result");
        assert!(parsed_probe.compile_ok);
        assert_eq!(parsed_probe.changed_lines, 42);
        assert_eq!(parsed_result.subtask_id, "subtask-1");
        assert!(parsed_result.success);
    }
}
