use super::{SandboxResult, hash_bytes};
use std::process::Output;
use std::time::Instant;

pub(super) fn from_output(
    command: &str,
    output: Output,
    started: Instant,
    violations: Vec<String>,
    unsafe_fallbacks: Vec<String>,
) -> SandboxResult {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined_output = if stderr.is_empty() {
        stdout.to_string()
    } else {
        format!("{stdout}\n--- stderr ---\n{stderr}")
    };
    SandboxResult {
        tool_id: command.to_string(),
        success: output.status.success(),
        output_hash: hash_bytes(combined_output.as_bytes()),
        output: combined_output,
        exit_code: output.status.code(),
        duration_ms: started.elapsed().as_millis() as u64,
        sandbox_violations: violations,
        unsafe_fallbacks,
    }
}
