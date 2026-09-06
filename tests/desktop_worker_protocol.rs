//! Real CLI subprocess protocol checks without invoking a Windows desktop.

#[test]
#[cfg(not(windows))]
fn desktop_worker_is_protocol_only_and_preserves_multiple_requests() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let workspace = tempfile::tempdir().unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args(["windows", "computer-use-worker"])
        .current_dir(workspace.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    input
        .write_all(b"{\"action\":\"status\"}\n{\"action\":\"list_apps\"}\n")
        .unwrap();
    drop(input);
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    let responses: Vec<codetether_agent::tool::ToolResult> = text
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(responses.len(), 2);
    assert!(responses.iter().all(|result| !result.success));
    assert!(
        output.stderr.is_empty(),
        "worker startup must not emit agent logs"
    );
    assert_eq!(std::fs::read_dir(workspace.path()).unwrap().count(), 0);
}
