use super::*;

/// Decode the last probe-prefixed log line.
///
/// # Arguments
/// * `logs` - Captured remote process output.
///
/// # Returns
/// The last matching record when valid; None if missing or malformed.
///
/// # Examples
/// ```
/// assert!(codetether_swarm_core::remote_protocol::latest_probe_from_logs("no probe").is_none());
/// ```
pub fn latest_probe_from_logs(logs: &str) -> Option<RemoteBranchProbe> {
    logs.lines()
        .rev()
        .find_map(|line| line.strip_prefix(SWARM_SUBTASK_PROBE_PREFIX))
        .and_then(|json| serde_json::from_str::<RemoteBranchProbe>(json).ok())
}

/// Decode the last result-prefixed log line.
///
/// # Arguments
/// * `logs` - Captured remote process output.
///
/// # Returns
/// The last matching record when valid; None if missing or malformed.
///
/// # Examples
/// ```
/// assert!(codetether_swarm_core::remote_protocol::result_from_logs("no result").is_none());
/// ```
pub fn result_from_logs(logs: &str) -> Option<SubTaskResult> {
    logs.lines()
        .rev()
        .find_map(|line| line.strip_prefix(SWARM_SUBTASK_RESULT_PREFIX))
        .and_then(|json| serde_json::from_str::<SubTaskResult>(json).ok())
}

/// Deduplicate the changed files recorded by a probe.
///
/// # Arguments
/// * `probe` - Remote branch observation.
///
/// # Returns
/// A set of changed path strings.
///
/// # Examples
/// See [RemoteBranchProbe] for construction.
pub fn probe_changed_files_set(probe: &RemoteBranchProbe) -> HashSet<String> {
    probe.changed_files.iter().cloned().collect()
}
