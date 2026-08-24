use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::session::SessionEvent;
use crate::tool::{Tool, command_session::Registry, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

#[path = "gate_retry_support.rs"]
mod support;

#[tokio::test]
async fn live_approval_retry_executes_once_and_replay_is_blocked() {
    let _lock = lock_env();
    let _queue = support::QueueGuard::new();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({
        "cmd": "true",
        "workdir": data.path().display().to_string(),
        "sandbox_permissions": "require_escalated",
        "__ct_session_id": "live-retry-session",
        "__ct_parent_workspace": data.path()
    });
    crate::tool::network_access::bind_trusted(&mut args, false);
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let workspace = data.path().to_path_buf();
    let gate = tokio::spawn(async move {
        super::gate::gate(&workspace, &tx, "call-1", "exec_command", args).await
    });
    let SessionEvent::ApprovalRequest(request) = rx.recv().await.expect("approval event") else {
        panic!("expected approval request");
    };
    crate::tui::app::state::approval_queue::push(request.clone());
    let mut app = crate::tui::app::state::App::default();
    let command = format!("/approve {} live retry", request.approval_id);
    assert!(crate::tui::app::input::approval_command::run(
        &mut app, &command
    ));
    let (approved, blocked) = gate.await.expect("gate task").into_parts();
    assert!(blocked.is_none());
    assert_eq!(approved["approval_id"], request.approval_id);
    assert!(
        crate::runtime_policy::evaluate_tool_invocation("exec_command", &approved)
            .await
            .is_none()
    );
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), None);
    let first = tool.execute(approved.clone()).await.expect("execution");
    assert!(first.success, "{}", first.output);
    let replay = tool.execute(approved).await.expect("replay");
    assert!(!replay.success);
}
