//! Verify process-based diagnostics are deferred until after the approval gate.

use serde_json::json;
use tokio::sync::mpsc;
use tokio::time::{Duration, timeout};

#[path = "gate_preflight_support.rs"]
mod support;

#[tokio::test]
async fn typescript_diagnostics_do_not_spawn_inside_approval_gate() {
    if which::which("typescript-language-server").is_err() {
        return;
    }
    let scope = support::Scope::new();
    let dir = scope.path();
    let path = dir.join("broken.ts");
    let args = json!({
        "path": path,
        "content": "export const broken: string = 42;\n",
        "__ct_parent_workspace": dir
    });
    let (tx, mut rx) = mpsc::channel(1);
    let workspace = dir.to_path_buf();
    let gate =
        tokio::spawn(
            async move { super::gate::gate(&workspace, &tx, "call-1", "write", args).await },
        );
    let event = timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("approval timeout")
        .expect("approval event");
    let crate::session::SessionEvent::ApprovalRequest(request) = event else {
        panic!("expected approval request");
    };
    assert!(crate::approval::live::decide(
        &request.approval_id,
        crate::approval::LiveApprovalDecision::Approved,
    ));
    let gate = timeout(Duration::from_secs(5), gate)
        .await
        .unwrap()
        .unwrap();
    let (_, blocked) = gate.into_parts();
    assert!(blocked.is_none());
}
