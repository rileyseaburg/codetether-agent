//! Goal-command validation, model rotation and pending approval presentation.
use super::{CommandHost, ensure_repository, propose};
/// Prepare an interactive goal for explicit objective approval.
/// # Arguments
/// * `host` - Native session, provider and approval-view bindings.
/// * `rest` - Task text following the goal command.
/// * `agent_count` - Existing default relay team size supplied by the caller.
/// # Returns
/// Completes with usage text or a pending objective awaiting user approval.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_relay_okr::interactive::CommandHost>(host: &mut H) {
/// codetether_relay_okr::interactive::execute(host, "repair parser", 3).await;
/// # }
/// ```
pub async fn execute<H: CommandHost>(host: &mut H, rest: &str, agent_count: usize) {
    let task = rest.trim();
    if task.is_empty() {
        host.set_status("Usage: /go <task description>".to_string());
        return;
    }
    let current_model = host.current_model();
    let model = crate::next_go_model(current_model);
    host.set_model(model.clone());
    if let Err(error) = host.save_session().await {
        tracing::warn!(error = %error, "Failed to save session after model swap");
    }
    ensure_repository(host.repository()).await;
    let pending = propose::<H>(task.to_string(), agent_count, model).await;
    host.push_system_message(pending.approval_prompt());
    host.set_pending(pending);
    host.set_status("OKR draft awaiting approval \u{2014} [A]pprove or [D]eny".to_string());
}
