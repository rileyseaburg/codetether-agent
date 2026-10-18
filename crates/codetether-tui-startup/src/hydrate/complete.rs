//! Apply resolved session state and startup notices.
use super::{Host, Session};
use codetether_session_resume::SessionLoadOutcome;
use codetether_workspace_snapshot::WorkspaceSnapshot;
/// Apply resolved session data and final startup presentation.
/// # Arguments
/// Receives frontend bindings, CLI network permission, the resolved session,
/// optional worker bridge, resolution outcome and workspace snapshot.
/// # Returns
/// Updates session/frontend state in startup order and presents outcome notices.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_tui_startup::hydrate::Session, H: codetether_tui_startup::hydrate::Host<S>>(host: &mut H, session: &mut S, outcome: codetether_session_resume::SessionLoadOutcome, workspace: codetether_workspace_snapshot::WorkspaceSnapshot) {
/// codetether_tui_startup::hydrate::complete(host, false, session, None, outcome, workspace);
/// # }
/// ```
pub fn complete<S: Session, H: Host<S>>(
    host: &mut H,
    allow_network: bool,
    session: &mut S,
    worker: Option<&H::Worker>,
    outcome: SessionLoadOutcome,
    workspace: WorkspaceSnapshot,
) {
    host.set_workspace(workspace);
    host.set_auto_apply(session.auto_apply());
    let network = session.allow_network() || allow_network;
    host.set_allow_network(network);
    host.set_autocomplete(session.autocomplete());
    host.set_use_worktree(session.use_worktree());
    host.set_session_id(session.id().to_string());
    session.set_allow_network(network);
    super::network::apply(network);
    let (source_id, has_older) = outcome.history_source(session.id());
    host.sync_messages(session, source_id, has_older);
    host.attach_worker(worker);
    host.refresh_suggestions();
    host.move_cursor_end();
    super::notice::session_banner::<S>(&outcome, host);
    let loaded = outcome.status();
    let policy = host.policy_summary();
    if let Some(policy) = policy.as_deref() {
        super::notice::policy_notice::<S>(host, policy);
    }
    host.set_status(match policy {
        Some(policy) => format!("{loaded} | {policy}"),
        None => loaded,
    });
}
